use chrono::{Local, Timelike};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
};
use std::fs;
use std::io::{Result, stdout};
use std::time::Duration;

const TITLE: &str = "Dinstachu";
const VERSION: &str = "0.0.1";
const COLOR_BORDER: Color = Color::Magenta;

fn main() -> Result<()> {
    let title_container = Span::styled(
        format!(" {} v{} ", TITLE, VERSION),
        Style::default().fg(Color::Green),
    );

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Read the contents of the current directory
    let entries = fs::read_dir(".")?
        .filter_map(|entry| entry.ok())
        .collect::<Vec<_>>();
    let mut rows: Vec<Row> = entries
        .iter()
        .map(|entry| {
            let name = entry.file_name();
            let metadata = entry.metadata().ok();
            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
            Row::new(vec![
                Cell::from(name.to_string_lossy().to_string()),
                Cell::from(format!("{} bytes", size)),
            ])
        })
        .collect();
    rows.insert(0, Row::new(vec![Cell::from(""), Cell::from("")]));

    // Main loop
    loop {
        terminal.draw(|f| {
            let area = f.area();

            let clock_container = Span::styled(
                Local::now().format(" %H:%M:%S ").to_string(),
                Style::default().fg(Color::Green),
            );

            // ---------- CHUNKS MAIN
            let chunks_main = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Percentage(100),
                        Constraint::Length(1),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(area);

            // ---------- BLOCK TOP
            let block_top = Block::default()
                .title_top(Line::from(title_container.clone()).centered())
                .title_top(Line::from(clock_container).right_aligned())
                // .title_alignment(Alignment::Right)
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
            let border_top = Paragraph::new(Text::raw(separator_top))
                .style(Style::default().fg(COLOR_BORDER))
                .alignment(ratatui::layout::Alignment::Left);
            f.render_widget(border_top, chunks_main[1]);

            // ---------- CHUNKS MIDDLE
            let chunks_middle = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Length(1),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(chunks_main[2]);

            let widths = [Constraint::Percentage(100)];
            let table_left = Table::new(rows.clone(), widths)
                .block(
                    Block::default()
                        .borders(Borders::LEFT)
                        .border_style(Style::default().fg(COLOR_BORDER)),
                )
                .widths(&[Constraint::Percentage(70), Constraint::Percentage(30)])
                .header(Row::new(vec![Cell::from("Name"), Cell::from("Metadata")]))
                .column_spacing(1);
            f.render_widget(table_left, chunks_middle[0]);

            let mut separator_middle = String::new();
            separator_middle.push_str(vertical);
            for _ in 1..(chunks_middle[0].height as usize) {
                separator_middle.push_str(vertical);
                separator_middle.push_str("\n");
            }
            separator_middle.push_str(vertical);
            let border_top = Paragraph::new(Text::raw(separator_middle))
                .style(Style::default().fg(COLOR_BORDER))
                .alignment(ratatui::layout::Alignment::Left);
            f.render_widget(border_top, chunks_middle[1]);

            let table_right = Table::new(rows.clone(), widths)
                .block(
                    Block::default()
                        .borders(Borders::RIGHT)
                        .border_style(Style::default().fg(COLOR_BORDER)),
                )
                .widths(&[Constraint::Percentage(70), Constraint::Percentage(30)])
                .header(Row::new(vec![Cell::from("Name"), Cell::from("Metadata")]))
                .column_spacing(1);
            f.render_widget(table_right, chunks_middle[2]);

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
            let border_bottom = Paragraph::new(Text::raw(separator_bottom))
                .style(Style::default().fg(COLOR_BORDER))
                .alignment(ratatui::layout::Alignment::Left);
            f.render_widget(border_bottom, chunks_main[3]);

            // ---------- BLOCK BOTTOM
            let block_bottom = Block::default()
                .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
                .border_style(Style::default().fg(COLOR_BORDER));
            f.render_widget(block_bottom, chunks_main[4]);
        })?;

        // Wait up to 500ms for an event
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::F(10) || key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
