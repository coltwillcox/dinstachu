use crate::constants::*;
use crate::utils::format_size;
use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Cell, Row},
};
use std::fs;
use std::io::Result;

pub fn load_directory_rows(path: &str) -> Result<Vec<Row>> {
    let mut entries = fs::read_dir(path)?.filter_map(|entry| entry.ok()).collect::<Vec<_>>();

    entries.sort_by(|a, b| {
        // let a_dir = a.path().is_dir();
        match (a.path().is_dir(), b.path().is_dir()) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().to_string_lossy().to_lowercase().cmp(&b.file_name().to_string_lossy().to_lowercase()),
        }
    });

    let mut rows: Vec<Row> = entries
        .iter()
        .map(|entry| {
            let path = entry.path();
            let is_dir = path.is_dir();
            let name = path.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string();
            let (dir_prefix, dir_suffix) = if is_dir { ("[", "]") } else { ("", "") };
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string();
            let size = if is_dir { "<DIR>".to_string() } else { format_size(entry.metadata().ok().map(|m| m.len()).unwrap_or(0)) };

            Row::new(vec![
                Cell::from(Line::from(vec![
                    Span::styled(dir_prefix, Style::default().fg(COLOR_DIRECTORY_FIX)),
                    Span::styled(name, Style::default().fg(if is_dir { COLOR_DIRECTORY } else { COLOR_FILE })),
                    Span::styled(dir_suffix, Style::default().fg(COLOR_DIRECTORY_FIX)),
                ])),
                Cell::from(ext),
                Cell::from(size),
            ])
        })
        .collect();

    rows.insert(0, Row::new(vec![Cell::from("..")]));
    Ok(rows)
}
