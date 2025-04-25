use crate::constants::*;
use chrono::Local;
use ratatui::{
    Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
};
use std::io::Result;

pub fn render_ui<B: Backend>(
    terminal: &mut Terminal<B>,
    rows_left: &[Row],
    rows_right: &[Row],
    state_left: &TableState,
    state_right: &TableState,
    is_left: bool,
) -> Result<()> {
    let title = Span::styled(format!(" {} v{} ", TITLE, VERSION), Style::default().fg(Color::Green));
    let clock = Span::styled(
        Local::now().format(" %H:%M:%S ").to_string(),
        Style::default().fg(Color::Green),
    );

    terminal.draw(|f| {
        let area = f.area();
        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Percentage(100),
                Constraint::Length(1),
                Constraint::Length(3),
            ])
            .split(area);

        let block_top = Block::default()
            .title_top(Line::from(title).centered())
            .title_top(Line::from(clock).right_aligned())
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
            .border_style(Style::default().fg(COLOR_BORDER));
        f.render_widget(block_top, chunks_main[0]);

        let separator_top = format!(
            "├{}┬{}┤",
            "─".repeat((area.width as usize - 3) / 2),
            "─".repeat((area.width as usize - 2) / 2)
        );
        f.render_widget(
            Paragraph::new(Text::raw(separator_top))
                .style(Style::default().fg(COLOR_BORDER))
                .alignment(Alignment::Left),
            chunks_main[1],
        );

        let chunks_middle = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Length(1), Constraint::Percentage(50)])
            .split(chunks_main[2]);

        let widths = [
            Constraint::Percentage(60),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ];
        let table_left = Table::new(rows_left.to_vec(), widths)
            .block(
                Block::default()
                    .borders(Borders::LEFT)
                    .border_style(Style::default().fg(COLOR_BORDER)),
            )
            .header(Row::new(vec![Cell::from("Name"), Cell::from("Ext"), Cell::from("Size")]))
            .row_highlight_style(
                Style::default()
                    .bg(if is_left {
                        COLOR_SELECTED_BACKGROUND
                    } else {
                        COLOR_SELECTED_BACKGROUND_INACTIVE
                    })
                    .fg(COLOR_SELECTED_FOREGROUND)
                    .add_modifier(Modifier::BOLD),
            )
            .column_spacing(1);
        f.render_stateful_widget(table_left, chunks_middle[0], &mut state_left.clone());

        let separator_vertical = Paragraph::new(Text::raw("│\n".repeat((chunks_middle[0].height - 1) as usize) + "│"))
            .style(Style::default().fg(COLOR_BORDER));
        f.render_widget(separator_vertical, chunks_middle[1]);

        let table_right = Table::new(rows_right.to_vec(), widths)
            .block(
                Block::default()
                    .borders(Borders::RIGHT)
                    .border_style(Style::default().fg(COLOR_BORDER)),
            )
            .header(Row::new(vec![Cell::from("Name"), Cell::from("Ext"), Cell::from("Size")]))
            .row_highlight_style(
                Style::default()
                    .bg(if !is_left {
                        COLOR_SELECTED_BACKGROUND
                    } else {
                        COLOR_SELECTED_BACKGROUND_INACTIVE
                    })
                    .fg(COLOR_SELECTED_FOREGROUND)
                    .add_modifier(Modifier::BOLD),
            )
            .column_spacing(1);
        f.render_stateful_widget(table_right, chunks_middle[2], &mut state_right.clone());

        let separator_bottom = format!(
            "├{}┴{}┤",
            "─".repeat((area.width as usize - 3) / 2),
            "─".repeat((area.width as usize - 2) / 2)
        );
        f.render_widget(
            Paragraph::new(Text::raw(separator_bottom))
                .style(Style::default().fg(COLOR_BORDER))
                .alignment(Alignment::Left),
            chunks_main[3],
        );

        let block_bottom = Block::default()
            .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
            .border_style(Style::default().fg(COLOR_BORDER));
        f.render_widget(block_bottom, chunks_main[4]);
    })?;

    Ok(())
}
