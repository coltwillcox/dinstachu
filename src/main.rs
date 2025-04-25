// TODO Separate dirs entries
// TODO Enter dir
// TODO Back dir
// TODO Read colors from config
use chrono::Local;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};
use std::fs;
use std::io::{Result, stdout};
use std::time::Duration;

const TITLE: &str = "Dinstachu";
const VERSION: &str = "0.0.1";
const COLOR_BORDER: Color = Color::Magenta;
const COLOR_SELECTED_BACKGROUND_INACTIVE: Color = Color::DarkGray;
const COLOR_SELECTED_BACKGROUND: Color = Color::LightMagenta;
const COLOR_SELECTED_FOREGROUND: Color = Color::Black;
const COLOR_DIRECTORY_FIX: Color = Color::Yellow;
const COLOR_DIRECTORY: Color = Color::LightYellow;
const COLOR_FILE: Color = Color::Gray;

fn main() -> Result<()> {
    let mut is_left = true;

    let title_container = Span::styled(format!(" {} v{} ", TITLE, VERSION), Style::default().fg(Color::Green));

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ---------- FOLDER READ
    let mut entries_left = fs::read_dir("/home/colt")?.filter_map(|entry| entry.ok()).collect::<Vec<_>>();
    entries_left.sort_by(|a, b| {
        // Sort: directories first, then by name (case-insensitive)
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();

        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => {
                let a_name = a.file_name().to_string_lossy().to_lowercase();
                let b_name = b.file_name().to_string_lossy().to_lowercase();
                a_name.cmp(&b_name)
            }
        }
    });
    let mut entries_right = fs::read_dir("/home/colt/.config")?.filter_map(|entry| entry.ok()).collect::<Vec<_>>();
    entries_right.sort_by(|a, b| {
        // Sort: directories first, then by name (case-insensitive)
        let a_is_dir = a.path().is_dir();
        let b_is_dir = b.path().is_dir();

        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => {
                let a_name = a.file_name().to_string_lossy().to_lowercase();
                let b_name = b.file_name().to_string_lossy().to_lowercase();
                a_name.cmp(&b_name)
            }
        }
    });

    // ---------- GENERATE ROWS
    let mut rows_left: Vec<Row> = entries_left
        .iter()
        .map(|entry| {
            let path = entry.path();
            let is_dir = path.is_dir();
            let name = path.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string();
            let mut dir_prefix = "";
            let mut dir_suffix = "";
            if is_dir {
                dir_prefix = "[";
                dir_suffix = "]";
            }
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string();
            let metadata = entry.metadata().ok();
            let size = if is_dir { String::from("<DIR>") } else { format_size(metadata.as_ref().map(|m| m.len()).unwrap_or(0)) };

            Row::new(vec![
                Cell::from(Line::from(vec![
                    Span::styled(dir_prefix, Style::default().fg(COLOR_DIRECTORY_FIX)),
                    Span::styled(name, Style::default().fg(if path.is_dir() { COLOR_DIRECTORY } else { COLOR_FILE })),
                    Span::styled(dir_suffix, Style::default().fg(COLOR_DIRECTORY_FIX)),
                ])),
                Cell::from(extension),
                Cell::from(size),
            ])
        })
        .collect();
    rows_left.insert(0, Row::new(vec![Cell::from("..")]));

    let mut rows_right: Vec<Row> = entries_right
        .iter()
        .map(|entry| {
            let path = entry.path();
            let is_dir = path.is_dir();
            let name = path.file_stem().and_then(|n| n.to_str()).unwrap_or("").to_string();
            let mut dir_prefix = "";
            let mut dir_suffix = "";
            if is_dir {
                dir_prefix = "[";
                dir_suffix = "]";
            }
            let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_string();
            let metadata = entry.metadata().ok();
            let size = if is_dir { String::from("<DIR>") } else { format_size(metadata.as_ref().map(|m| m.len()).unwrap_or(0)) };

            Row::new(vec![
                Cell::from(Line::from(vec![
                    Span::styled(dir_prefix, Style::default().fg(COLOR_DIRECTORY_FIX)),
                    Span::styled(name, Style::default().fg(if path.is_dir() { COLOR_DIRECTORY } else { COLOR_FILE })),
                    Span::styled(dir_suffix, Style::default().fg(COLOR_DIRECTORY_FIX)),
                ])),
                Cell::from(extension),
                Cell::from(size),
            ])
        })
        .collect();
    rows_right.insert(0, Row::new(vec![Cell::from("..")]));

    // ---------- STATES
    let mut state_left = TableState::default();
    state_left.select(Some(1));
    let mut state_right = TableState::default();
    state_right.select(Some(1));

    // ---------- MAIN LOOP
    loop {
        terminal.draw(|f| {
            let area = f.area();

            let clock_container = Span::styled(Local::now().format(" %H:%M:%S ").to_string(), Style::default().fg(Color::Green));

            // ---------- CHUNKS MAIN
            let chunks_main = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Length(1), Constraint::Percentage(100), Constraint::Length(1), Constraint::Length(3)].as_ref())
                .split(area);

            // ---------- BLOCK TOP
            let block_top = Block::default()
                .title_top(Line::from(title_container.clone()).centered())
                .title_top(Line::from(clock_container).right_aligned())
                .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
                .border_style(Style::default().fg(COLOR_BORDER));
            f.render_widget(block_top, chunks_main[0]);

            let joint_left = "├";
            let joint_right = "┤";
            let joint_top: &str = "┬";
            let joint_bottom: &str = "┴";
            let horizontal = "─";
            let vertical = "│";

            // ---------- SEPARATOR TOP
            let mut separator_top = String::new();
            separator_top.push_str(joint_left);
            for i in 0..(area.width as usize - 2) {
                if i == (area.width as usize - 3) / 2 {
                    separator_top.push_str(joint_top);
                } else {
                    separator_top.push_str(horizontal);
                }
            }
            separator_top.push_str(joint_right);
            let border_top = Paragraph::new(Text::raw(separator_top)).style(Style::default().fg(COLOR_BORDER)).alignment(ratatui::layout::Alignment::Left);
            f.render_widget(border_top, chunks_main[1]);

            // ---------- CHUNKS MIDDLE
            let chunks_middle = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Length(1), Constraint::Percentage(50)].as_ref())
                .split(chunks_main[2]);

            // ---------- TABLE LEFT
            let widths = [Constraint::Percentage(60), Constraint::Percentage(20), Constraint::Percentage(20)];
            let table_left = Table::new(rows_left.clone(), widths)
                .block(Block::default().borders(Borders::LEFT).border_style(Style::default().fg(COLOR_BORDER)))
                .header(Row::new(vec![Cell::from("Name"), Cell::from("Ext"), Cell::from("Size")]))
                .row_highlight_style(
                    Style::default()
                        .bg(if is_left { COLOR_SELECTED_BACKGROUND } else { COLOR_SELECTED_BACKGROUND_INACTIVE })
                        .fg(COLOR_SELECTED_FOREGROUND)
                        .add_modifier(Modifier::BOLD),
                )
                .column_spacing(1);
            f.render_stateful_widget(table_left, chunks_middle[0], &mut state_left);

            // ---------- SEPARATOR MIDDLE
            let mut separator_middle = String::new();
            separator_middle.push_str(vertical);
            for _ in 1..(chunks_middle[0].height as usize) {
                separator_middle.push_str(vertical);
                separator_middle.push_str("\n");
            }
            separator_middle.push_str(vertical);
            let border_top = Paragraph::new(Text::raw(separator_middle)).style(Style::default().fg(COLOR_BORDER)).alignment(ratatui::layout::Alignment::Left);
            f.render_widget(border_top, chunks_middle[1]);

            // ---------- TABLE RIGHT
            let table_right = Table::new(rows_right.clone(), widths)
                .block(Block::default().borders(Borders::RIGHT).border_style(Style::default().fg(COLOR_BORDER)))
                .header(Row::new(vec![Cell::from("Name"), Cell::from("Ext"), Cell::from("Size")]))
                .row_highlight_style(
                    Style::default()
                        .bg(if !is_left { COLOR_SELECTED_BACKGROUND } else { COLOR_SELECTED_BACKGROUND_INACTIVE })
                        .fg(COLOR_SELECTED_FOREGROUND)
                        .add_modifier(Modifier::BOLD),
                )
                .column_spacing(1);
            f.render_stateful_widget(table_right, chunks_middle[2], &mut state_right);

            // ---------- SEPARATOR BOTTOM
            let mut separator_bottom = String::new();
            separator_bottom.push_str(joint_left);
            for i in 0..(area.width as usize - 2) {
                if i == (area.width as usize - 3) / 2 {
                    separator_bottom.push_str(joint_bottom);
                } else {
                    separator_bottom.push_str(horizontal);
                }
            }
            separator_bottom.push_str(joint_right);
            let border_bottom = Paragraph::new(Text::raw(separator_bottom)).style(Style::default().fg(COLOR_BORDER)).alignment(ratatui::layout::Alignment::Left);
            f.render_widget(border_bottom, chunks_main[3]);

            // ---------- BLOCK BOTTOM
            let block_bottom = Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT).border_style(Style::default().fg(COLOR_BORDER));
            f.render_widget(block_bottom, chunks_main[4]);
        })?;

        // ---------- KEY COMMANDS
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::F(10) => break,
                    KeyCode::Down => {
                        if is_left {
                            if let Some(i) = state_left.selected() {
                                let next = if i >= rows_left.len() - 1 { 0 } else { i + 1 };
                                state_left.select(Some(next));
                            }
                        } else {
                            if let Some(i) = state_right.selected() {
                                let next = if i >= rows_right.len() - 1 { 0 } else { i + 1 };
                                state_right.select(Some(next));
                            }
                        }
                    }
                    KeyCode::Up => {
                        if is_left {
                            if let Some(i) = state_left.selected() {
                                let prev = if i == 0 { rows_left.len() - 1 } else { i - 1 };
                                state_left.select(Some(prev));
                            }
                        } else {
                            if let Some(i) = state_right.selected() {
                                let prev = if i == 0 { rows_right.len() - 1 } else { i - 1 };
                                state_right.select(Some(prev));
                            }
                        }
                    }
                    KeyCode::Tab => is_left = !is_left,
                    _ => {}
                }
            }
        }
    }

    // ---------- CLEANUP
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

/// Converts bytes to human-readable format with binary prefixes (KiB, MiB, etc.)
fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.0} {}", size, UNITS[unit_index])
}
