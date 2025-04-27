use crate::constants::*;
use std::path::PathBuf;

// Converts bytes to human-readable format with binary prefixes (KiB, MiB, etc.)
pub fn format_size(bytes: u64) -> String {
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.0} {}", size, UNITS[unit_index])
}

pub fn limit_path_string(path_buf: &PathBuf, n: usize) -> String {
    let path_string = path_buf.display().to_string();
    if path_string.len() <= n { path_string } else { format!("...{}", &path_string[(path_string.len() - n)..]) }
}
