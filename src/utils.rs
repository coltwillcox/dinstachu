use crate::constants::*;

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
