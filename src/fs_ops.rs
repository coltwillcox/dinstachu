use crate::app::Item;
use crate::utils::format_size;
use std::env;
use std::fs::{DirEntry, create_dir, read_dir, remove_dir_all, remove_file, rename};
use std::io::Error;
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
        _ => a.file_name().to_string_lossy().to_lowercase().cmp(&b.file_name().to_string_lossy().to_lowercase()),
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
        });
    }

    for entry in &entries {
        let path = entry.path();
        let is_dir = path.is_dir();
        let name_full = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let name = if is_dir { name_full.clone() } else { path.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string() };
        let extension = if is_dir { "".to_string() } else { path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string() };
        let size = if is_dir { "<DIR>".to_string() } else { format_size(entry.metadata().ok().map(|m| m.len()).unwrap_or(0)) };

        children.push(Item {
            name_full: name_full.clone(),
            name: name.clone(),
            extension: extension.clone(),
            is_dir,
            size: size.clone(),
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
            if parent.as_os_str().as_bytes().contains(&b':') && parent.parent().is_none() {
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
