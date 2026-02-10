use crate::app::Item;
use crate::utils::format_size;
use chrono::Local;
use std::env;
use std::fs::{self, DirEntry, File, create_dir, read_dir, remove_dir_all, remove_file, rename};
use std::io::{Error, Read, Write};
use std::path::PathBuf;

pub fn load_directory_rows(path: &PathBuf) -> Result<Vec<Item>, Error> {
    let mut entries: Vec<DirEntry>;
    let entries_result = read_dir(path);

    match entries_result {
        Ok(dir) => {
            entries = dir.filter_map(|entry| entry.ok()).collect::<Vec<_>>();
        }
        Err(e) => {
            return Err(e);
        }
    }

    entries.sort_by(|a, b| match (a.path().is_dir(), b.path().is_dir()) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        (true, true) => a.file_name().to_string_lossy().to_lowercase().cmp(&b.file_name().to_string_lossy().to_lowercase()),
        (false, false) => {
            let ext_a = a.path().extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
            let ext_b = b.path().extension().map(|e| e.to_string_lossy().to_lowercase()).unwrap_or_default();
            ext_a.cmp(&ext_b).then_with(|| {
                a.file_name().to_string_lossy().to_lowercase().cmp(&b.file_name().to_string_lossy().to_lowercase())
            })
        }
    });

    let mut children = Vec::<Item>::new();

    // Don't add ".." on root folder.
    if path.parent().is_some() {
        children.push(Item {
            name_full: "..".to_string(),
            name: "..".to_string(),
            extension: "".to_string(),
            is_dir: true,
            size: "".to_string(),
            modified: "".to_string(),
        });
    }

    for entry in &entries {
        let path = entry.path();
        let is_dir = path.is_dir();
        let name_full = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let name = if is_dir { name_full.clone() } else { path.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string() };
        let extension = if is_dir { "".to_string() } else { path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string() };
        let metadata = entry.metadata().ok();
        let size = if is_dir { "<DIR>".to_string() } else { format_size(metadata.as_ref().map(|m| m.len()).unwrap_or(0)) };
        let modified = metadata.as_ref()
            .and_then(|m| m.modified().ok())
            .map(|t| {
                let dt: chrono::DateTime<Local> = t.into();
                dt.format("%d/%m/%y %H:%M").to_string()
            })
            .unwrap_or_default();

        children.push(Item {
            name_full: name_full.clone(),
            name: name.clone(),
            extension: extension.clone(),
            is_dir,
            size: size.clone(),
            modified,
        });
    }

    Ok(children)
}

pub fn get_root_dir() -> Result<PathBuf, std::io::Error> {
    env::current_dir().map(|path_current| {
        let mut path = path_current;
        while let Some(parent) = path.parent() {
            if parent.components().count() == 1 {
                // This likely indicates the root directory on Unix-like systems.
                return parent.to_path_buf();
            }
            #[cfg(windows)]
            if parent.to_string_lossy().contains(':') && parent.parent().is_none() {
                // This likely indicates the root of a drive on Windows (e.g., "C:").
                return parent.to_path_buf();
            }
            path = parent.to_path_buf();
        }
        path // Fallback to the current directory if no clear root is found.
    })
}

pub fn get_current_dir() -> Result<PathBuf, std::io::Error> {
    match env::current_dir() {
        Ok(path) => Ok(path),
        Err(e) => Err(e),
    }
}

pub fn rename_path(original_path: PathBuf, new_path: PathBuf) -> Result<(), Error> {
    rename(original_path, new_path)?;
    Ok(())
}

pub fn delete_path(path: PathBuf, is_dir: bool) -> Result<(), Error> {
    if is_dir {
        remove_dir_all(path)?;
    } else {
        remove_file(path)?;
    }
    Ok(())
}

pub fn create_directory(path: PathBuf) -> Result<(), Error> {
    create_dir(path)?;
    Ok(())
}

pub fn copy_path(source: PathBuf, dest: PathBuf, is_dir: bool) -> Result<(), Error> {
    if is_dir {
        copy_dir_recursive(&source, &dest)
    } else {
        copy_file_content(&source, &dest)
    }
}

/// Copy file content without trying to preserve Unix permissions.
/// This works across filesystems (e.g., ext4 to exFAT) where permission
/// preservation would fail with EPERM.
fn copy_file_content(source: &PathBuf, dest: &PathBuf) -> Result<(), Error> {
    let mut src_file = File::open(source)?;
    let mut dst_file = File::create(dest)?;

    let mut buffer = [0u8; 64 * 1024]; // 64KB buffer
    loop {
        let bytes_read = src_file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        dst_file.write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}

fn copy_dir_recursive(source: &PathBuf, dest: &PathBuf) -> Result<(), Error> {
    fs::create_dir_all(dest)?;

    for entry in read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry.file_name();
        let dest_path = dest.join(&file_name);

        if entry_path.is_dir() {
            copy_dir_recursive(&entry_path, &dest_path)?;
        } else {
            copy_file_content(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}

pub fn move_path(source: PathBuf, dest: PathBuf, is_dir: bool) -> Result<(), Error> {
    // Try rename first (fast, same filesystem)
    match rename(&source, &dest) {
        Ok(_) => Ok(()),
        Err(e) => {
            // Check for cross-device error:
            // - EXDEV (18) on Linux/macOS/Unix
            // - ERROR_NOT_SAME_DEVICE (17) on Windows
            let is_cross_device = match e.raw_os_error() {
                Some(18) => true,  // EXDEV on Unix
                Some(17) => true,  // ERROR_NOT_SAME_DEVICE on Windows
                _ => false,
            };

            if is_cross_device {
                // Cross-device move: copy then delete
                copy_path(source.clone(), dest.clone(), is_dir)?;

                // Delete source - if this fails, the copy succeeded but source remains
                if let Err(del_err) = delete_path(source, is_dir) {
                    return Err(Error::new(
                        del_err.kind(),
                        format!(
                            "Move partially complete: copied to {} but failed to delete source: {}",
                            dest.display(),
                            del_err
                        ),
                    ));
                }
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}

pub fn calculate_dir_size(path: &PathBuf) -> Result<u64, Error> {
    let mut total_size = 0u64;

    for entry in read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        if entry_path.is_dir() {
            // Continue on subdirectory errors, just skip that dir
            if let Ok(size) = calculate_dir_size(&entry_path) {
                total_size += size;
            }
        } else {
            total_size += entry.metadata()?.len();
        }
    }

    Ok(total_size)
}
