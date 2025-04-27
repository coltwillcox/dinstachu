use crate::constants::*;
use crate::utils::format_size;
use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::{Cell, Row},
};
use std::fs;
use std::io::Result;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub extension: String,
    pub is_dir: bool,
    pub size: String,
}

pub fn load_directory_rows<'a>(path: &PathBuf) -> Result<(Vec<Row<'a>>, Vec<Item>)> {
    let mut entries = fs::read_dir(path)?.filter_map(|entry| entry.ok()).collect::<Vec<_>>();

    entries.sort_by(|a, b| match (a.path().is_dir(), b.path().is_dir()) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.file_name().to_string_lossy().to_lowercase().cmp(&b.file_name().to_string_lossy().to_lowercase()),
    });

    let mut children = Vec::<Item>::new();
    let mut rows = Vec::<Row>::new();

    // Don't add ".." on root folder
    if path.parent().is_some() {
        children.push(Item {
            name: "..".to_string(),
            extension: "".to_string(),
            is_dir: true,
            size: "".to_string(),
        });
        rows.insert(0, Row::new(vec![Cell::from(Span::styled("", Style::default().fg(COLOR_FILE))), Cell::from(Span::styled("..", Style::default().fg(COLOR_FILE)))]));
    }

    for entry in &entries {
        let path = entry.path();
        let is_dir = path.is_dir();
        let name = if is_dir {
            path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()
        } else {
            path.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string()
        };
        let (dir_prefix, dir_suffix) = if is_dir { ("[", "]") } else { ("", "") };
        let extension = if is_dir { "".to_string() } else { path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string() };
        let size = if is_dir { "<DIR>".to_string() } else { format_size(entry.metadata().ok().map(|m| m.len()).unwrap_or(0)) };
        let icon = if is_dir { ICON_FOLDER } else { ICON_FILE };

        children.push(Item {
            name: name.clone(),
            extension: extension.clone(),
            is_dir,
            size: size.clone(),
        });
        rows.push(Row::new(vec![
            Cell::from(Span::styled(icon, Style::default().fg(if is_dir { COLOR_DIRECTORY } else { COLOR_FILE }))),
            Cell::from(Line::from(vec![
                Span::styled(dir_prefix, Style::default().fg(COLOR_DIRECTORY_FIX)),
                Span::styled(name, Style::default().fg(if is_dir { COLOR_DIRECTORY } else { COLOR_FILE })),
                Span::styled(dir_suffix, Style::default().fg(COLOR_DIRECTORY_FIX)),
            ])),
            Cell::from(Span::styled(extension, Style::default().fg(if is_dir { COLOR_DIRECTORY } else { COLOR_FILE }))),
            Cell::from(Span::styled(size, Style::default().fg(if is_dir { COLOR_DIRECTORY } else { COLOR_FILE }))),
        ]));
    }

    Ok((rows, children))
}
