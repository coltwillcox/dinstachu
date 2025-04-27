use crate::constants::*;
use chrono::Local;
use ratatui::{
    Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState},
};
use std::io::Result;
use std::path::PathBuf;

pub fn render_ui<B: Backend>(terminal: &mut Terminal<B>, dir_left: &mut PathBuf, dir_right: &mut PathBuf, rows_left: &[Row], rows_right: &[Row], state_left: &TableState, state_right: &TableState, is_left: bool) -> Result<u16> {
    let mut page_size: u16 = 0;

    let title = Span::styled(format!(" {} v{} ", TITLE, VERSION), Style::default().fg(COLOR_TITLE));
    let clock = Span::styled(Local::now().format(" %H:%M:%S ").to_string(), Style::default().fg(COLOR_TITLE));

    terminal.draw(|f| {
        let area = f.area();
        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(1), Constraint::Percentage(100), Constraint::Length(1), Constraint::Length(3)])
            .split(area);

        let block_top = Block::default()
            .title_top(Line::from(title).centered())
            .title_top(Line::from(clock).right_aligned())
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
            .border_style(Style::default().fg(COLOR_BORDER));
        f.render_widget(block_top, chunks_main[0]);

        let length_left = ((area.width as usize).saturating_sub(3)) / 2;
        let length_right = ((area.width as usize).saturating_sub(2)) / 2;
        let path_left = limit_path_string(&dir_left, length_left.saturating_sub(5));
        let path_right = limit_path_string(&dir_right, length_right.saturating_sub(5));
        let border_1 = format!("{}", "├─");
        let border_2 = format!("{}", path_left);
        let border_3 = format!("{}{}", "─".repeat(length_left.saturating_sub(path_left.len().saturating_add(2))), "─┬─");
        let border_4 = format!("{}", path_right);
        let border_5 = format!("{}{}", "─".repeat(length_right.saturating_sub(path_right.len().saturating_add(2))), "─┤".to_string());

        f.render_widget(
            Paragraph::new(Line::from(vec![
                Span::styled(border_1, Style::default().fg(COLOR_BORDER)),
                Span::styled(border_2, Style::default().fg(COLOR_DIRECTORY)),
                Span::styled(border_3, Style::default().fg(COLOR_BORDER)),
                Span::styled(border_4, Style::default().fg(COLOR_DIRECTORY)),
                Span::styled(border_5, Style::default().fg(COLOR_BORDER)),
            ])),
            chunks_main[1],
        );

        let chunks_middle = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Length(1), Constraint::Percentage(50)])
            .split(chunks_main[2]);

        let widths = [Constraint::Percentage(60), Constraint::Percentage(20), Constraint::Percentage(20)];
        let table_left = Table::new(rows_left.to_vec(), widths)
            .block(Block::default().borders(Borders::LEFT).border_style(Style::default().fg(COLOR_BORDER)))
            .header(Row::new(vec![
                Cell::from(Span::styled("Name", Style::default().fg(COLOR_COLUMNS))),
                Cell::from(Span::styled("Ext", Style::default().fg(COLOR_COLUMNS))),
                Cell::from(Span::styled("Size", Style::default().fg(COLOR_COLUMNS))),
            ]))
            .row_highlight_style(
                Style::default()
                    .bg(if is_left { COLOR_SELECTED_BACKGROUND } else { COLOR_SELECTED_BACKGROUND_INACTIVE })
                    .fg(COLOR_SELECTED_FOREGROUND)
                    .add_modifier(Modifier::BOLD),
            )
            .column_spacing(1);
        f.render_stateful_widget(table_left, chunks_middle[0], &mut state_left.clone());

        let separator_vertical = Paragraph::new(Text::raw("│\n".repeat((chunks_middle[0].height - 1) as usize) + "│")).style(Style::default().fg(COLOR_BORDER));
        f.render_widget(separator_vertical, chunks_middle[1]);

        let table_right = Table::new(rows_right.to_vec(), widths)
            .block(Block::default().borders(Borders::RIGHT).border_style(Style::default().fg(COLOR_BORDER)))
            .header(Row::new(vec![
                Cell::from(Span::styled("Name", Style::default().fg(COLOR_COLUMNS))),
                Cell::from(Span::styled("Ext", Style::default().fg(COLOR_COLUMNS))),
                Cell::from(Span::styled("Size", Style::default().fg(COLOR_COLUMNS))),
            ]))
            .row_highlight_style(
                Style::default()
                    .bg(if !is_left { COLOR_SELECTED_BACKGROUND } else { COLOR_SELECTED_BACKGROUND_INACTIVE })
                    .fg(COLOR_SELECTED_FOREGROUND)
                    .add_modifier(Modifier::BOLD),
            )
            .column_spacing(1);
        f.render_stateful_widget(table_right, chunks_middle[2], &mut state_right.clone());

        let separator_bottom = format!("├{}┴{}┤", "─".repeat(((area.width as usize).saturating_sub(3)) / 2), "─".repeat(((area.width as usize).saturating_sub(2)) / 2));
        f.render_widget(Paragraph::new(Text::raw(separator_bottom)).style(Style::default().fg(COLOR_BORDER)).alignment(Alignment::Left), chunks_main[3]);

        let block_bottom = Block::default().borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT).border_style(Style::default().fg(COLOR_BORDER));
        f.render_widget(block_bottom, chunks_main[4]);

        page_size = chunks_middle[0].height;

        // TODO Popup POC
        let show_popup = false;
        if show_popup {
            let popup_block = Block::default()
                .title(Line::from(Span::styled(format!(" {} ", "Popup title"), Style::default().fg(COLOR_TITLE))).centered())
                .borders(Borders::ALL)
                .style(Style::default().fg(COLOR_BORDER).bg(Color::Black));
            let area = centered_rect(60, 20, f.area()); // Calculate centered rect
            // `Clear` is important to draw over the existing content
            f.render_widget(Clear::default(), area);
            f.render_widget(popup_block, area);
            f.render_widget(
                Paragraph::new("This is a popup!").alignment(Alignment::Center).style(Style::default().fg(COLOR_TITLE)),
                area.inner(Margin { vertical: 2, horizontal: 2 }), // Add some padding
            );
        }
    })?;

    Ok(page_size)
}

fn limit_path_string(path_buf: &PathBuf, n: usize) -> String {
    let path_string = path_buf.display().to_string();
    if path_string.len() <= n { path_string } else { format!("{}...", &path_string[..n]) }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage((100 - percent_y) / 2), Constraint::Percentage(percent_y), Constraint::Percentage((100 - percent_y) / 2)])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage((100 - percent_x) / 2), Constraint::Percentage(percent_x), Constraint::Percentage((100 - percent_x) / 2)])
        .split(popup_layout[1])[1]
}
