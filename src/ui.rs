use crate::app::AppState;
use crate::constants::*;
use crate::utils::*;
use chrono::Local;
use ratatui::{
    Terminal,
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table},
};
use std::path::PathBuf;

pub fn render_ui<B: Backend>(terminal: &mut Terminal<B>, app_state: &mut AppState) {
    let _ = terminal.draw(|f| {
        let area = f.area();
        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(1), Constraint::Percentage(100), Constraint::Length(1), Constraint::Length(3)])
            .split(area);

        render_top_panel(f, chunks_main[0]);
        render_path_bar(f, chunks_main[1], &app_state.dir_left, &app_state.dir_right, area.width);
        app_state.page_size = render_file_tables(f, chunks_main[2], app_state);
        render_bottom_panel(f, chunks_main[3], area.width);
        render_fkey_bar(f, chunks_main[4]);

        if app_state.is_error_displayed {
            render_error_popup(f, area, app_state);
        } else if app_state.is_f1_displayed {
            render_help_popup(f, area);
        } else if app_state.is_f7_displayed {
            render_create_popup(f, area);
        }
    });
}

fn render_top_panel(f: &mut ratatui::Frame<'_>, area: Rect) {
    let logo = Span::styled(format!(" {} ", ICON_LOGO), Style::default().fg(COLOR_TITLE));
    let title = Span::styled(format!(" {} v{} ", TITLE, VERSION), Style::default().fg(COLOR_TITLE));
    let clock = Span::styled(Local::now().format(" %H:%M:%S ").to_string(), Style::default().fg(COLOR_TITLE));

    let block_top = Block::default()
        .title_top(Line::from(logo).left_aligned())
        .title_top(Line::from(title).centered())
        .title_top(Line::from(clock).right_aligned())
        .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
        .border_style(Style::default().fg(COLOR_BORDER));

    f.render_widget(block_top, area);
}

fn render_path_bar(f: &mut ratatui::Frame<'_>, area: Rect, dir_left: &PathBuf, dir_right: &PathBuf, total_width: u16) {
    let length_left = ((total_width as usize).saturating_sub(3)) / 2;
    let length_right = ((total_width as usize).saturating_sub(2)) / 2;

    let path_left = limit_path_string(dir_left, length_left.saturating_sub(8));
    let path_right = limit_path_string(dir_right, length_right.saturating_sub(8));

    let border_line = vec![
        Span::styled(format!("{}", "├──"), Style::default().fg(COLOR_BORDER)),
        Span::styled(format!(" {} ", path_left), Style::default().fg(COLOR_DIRECTORY)),
        Span::styled(format!("{}{}", "─".repeat(length_left.saturating_sub(path_left.len().saturating_add(5))), "─┬──"), Style::default().fg(COLOR_BORDER)),
        Span::styled(format!(" {} ", path_right), Style::default().fg(COLOR_DIRECTORY)),
        Span::styled(format!("{}{}", "─".repeat(length_right.saturating_sub(path_right.len().saturating_add(5))), "─┤"), Style::default().fg(COLOR_BORDER)),
    ];

    f.render_widget(Paragraph::new(Line::from(border_line)), area);
}

fn render_file_tables(f: &mut ratatui::Frame<'_>, chunk: Rect, app_state: &mut AppState) -> u16 {
    let chunks = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(50), Constraint::Length(1), Constraint::Percentage(50)]).split(chunk);

    let widths = [Constraint::Length(2), Constraint::Percentage(70), Constraint::Length(1), Constraint::Percentage(15), Constraint::Length(1), Constraint::Percentage(15)];

    let table_style = |active: bool| {
        Style::default()
            .bg(if active {
                if app_state.is_f2_displayed { COLOR_RENAME_BACKGROUND } else { COLOR_SELECTED_BACKGROUND }
            } else {
                COLOR_SELECTED_BACKGROUND_INACTIVE
            })
            .fg(COLOR_SELECTED_FOREGROUND)
            .add_modifier(Modifier::BOLD)
    };

    let rows_left = children_to_rows(&app_state, true);
    let table_left = Table::new(rows_left.to_vec(), widths.clone())
        .block(Block::default().borders(Borders::LEFT).border_style(Style::default().fg(COLOR_BORDER)))
        .header(make_header_row())
        .row_highlight_style(table_style(app_state.is_left_active))
        .column_spacing(1);
    f.render_stateful_widget(table_left, chunks[0], &mut app_state.state_left);

    let separator_vertical = Paragraph::new(Text::raw("│\n".repeat((chunks[0].height - 1) as usize) + "│")).style(Style::default().fg(COLOR_BORDER));
    f.render_widget(separator_vertical, chunks[1]);

    let rows_right = children_to_rows(&app_state, false);
    let table_right = Table::new(rows_right.to_vec(), widths)
        .block(Block::default().borders(Borders::RIGHT).border_style(Style::default().fg(COLOR_BORDER)))
        .header(make_header_row())
        .row_highlight_style(table_style(!app_state.is_left_active))
        .column_spacing(1);
    f.render_stateful_widget(table_right, chunks[2], &mut app_state.state_right);

    chunks[0].height
}

fn children_to_rows<'a>(app_state: &AppState, is_left: bool) -> Vec<Row<'static>> {
    let mut rows = Vec::<Row<'static>>::new();

    let children = if is_left { &app_state.children_left.clone() } else { &app_state.children_right.clone() };

    let is_renaming_current_side = app_state.is_f2_displayed && (app_state.is_left_active == is_left);
    let selected_index = if is_left { app_state.state_left.selected().unwrap() } else { app_state.state_right.selected().unwrap() };

    for (index, child) in children.iter().enumerate() {
        let is_renaming_current_item = is_renaming_current_side && (index == selected_index);
        let icon = if child.is_dir { ICON_FOLDER } else { ICON_FILE };
        let (dir_prefix, dir_suffix) = if child.is_dir { ("[", "]") } else { ("", "") };

        let name = if is_renaming_current_item { app_state.get_rename_input() } else { child.name.clone() };
        let extension = if is_renaming_current_item { String::new() } else { child.extension.clone() };

        rows.push(Row::new(vec![
            Cell::from(Span::styled(icon, Style::default().fg(if child.is_dir { COLOR_DIRECTORY } else { COLOR_FILE }))),
            Cell::from(Line::from(vec![
                Span::styled(dir_prefix, Style::default().fg(COLOR_DIRECTORY_FIX)),
                Span::styled(name, Style::default().fg(if child.is_dir { COLOR_DIRECTORY } else { COLOR_FILE })),
                Span::styled(dir_suffix, Style::default().fg(COLOR_DIRECTORY_FIX)),
            ])),
            Cell::from(Span::styled("│", Style::default().fg(COLOR_BORDER))),
            Cell::from(Span::styled(extension, Style::default().fg(if child.is_dir { COLOR_DIRECTORY } else { COLOR_FILE }))),
            Cell::from(Span::styled("│", Style::default().fg(COLOR_BORDER))),
            Cell::from(Span::styled(child.size.clone(), Style::default().fg(if child.is_dir { COLOR_DIRECTORY } else { COLOR_FILE }))),
        ]));
    }

    return rows;
}

fn make_header_row() -> Row<'static> {
    Row::new(vec![
        Cell::from(Span::styled("", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("Name", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("Ext", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("Size", Style::default().fg(COLOR_COLUMNS))),
    ])
}

fn render_bottom_panel(f: &mut ratatui::Frame<'_>, area: Rect, total_width: u16) {
    let separator = format!("├{}┴{}┤", "─".repeat(((total_width as usize).saturating_sub(3)) / 2), "─".repeat(((total_width as usize).saturating_sub(2)) / 2));
    f.render_widget(Paragraph::new(Text::raw(separator)).style(Style::default().fg(COLOR_BORDER)), area);
}

fn render_fkey_bar(f: &mut ratatui::Frame<'_>, area: Rect) {
    let block_bottom = Block::default()
        .title_bottom(Line::from(Span::styled(" F1 Help ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F2 Rename ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F3 ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F4 ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F5 ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F6 ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F7 Create ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F8 ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F9 ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F10 Quit ", Style::default().fg(COLOR_TITLE))).centered())
        .borders(Borders::LEFT | Borders::BOTTOM | Borders::RIGHT)
        .border_style(Style::default().fg(COLOR_BORDER));
    f.render_widget(block_bottom, area);
}

fn render_error_popup(f: &mut ratatui::Frame<'_>, area: Rect, app_state: &mut AppState) {
    let popup_area = centered_rect(60, 20, area);
    let popup_block = Block::default()
        .title(Line::from(Span::styled(" Error ", Style::default().fg(COLOR_TITLE))).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_BORDER));

    f.render_widget(Clear::default(), popup_area); // `Clear` is important to draw over the existing content.
    f.render_widget(popup_block, popup_area);

    f.render_widget(
        Paragraph::new(app_state.error_message.clone()).alignment(Alignment::Center).style(Style::default().fg(COLOR_TITLE)),
        popup_area.inner(Margin { vertical: 2, horizontal: 2 }),
    );
}

fn render_help_popup(f: &mut ratatui::Frame<'_>, area: Rect) {
    let popup_area = centered_rect(60, 20, area);
    let popup_block = Block::default()
        .title(Line::from(Span::styled(" Help/About ", Style::default().fg(COLOR_TITLE))).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_BORDER));

    f.render_widget(Clear::default(), popup_area);
    f.render_widget(popup_block, popup_area);

    f.render_widget(
        Paragraph::new("F1 - This help").alignment(Alignment::Center).style(Style::default().fg(COLOR_TITLE)),
        popup_area.inner(Margin { vertical: 2, horizontal: 2 }),
    );
    f.render_widget(
        Paragraph::new("F2 - Rename folder/file").alignment(Alignment::Center).style(Style::default().fg(COLOR_TITLE)),
        popup_area.inner(Margin { vertical: 3, horizontal: 2 }),
    );
    f.render_widget(Paragraph::new("F10 - Quit").alignment(Alignment::Center).style(Style::default().fg(COLOR_TITLE)), popup_area.inner(Margin { vertical: 4, horizontal: 2 }));
}

fn render_create_popup(f: &mut ratatui::Frame<'_>, area: Rect) {
    let popup_area = centered_rect(60, 20, area);
    let popup_block = Block::default()
        .title(Line::from(Span::styled(" Create ", Style::default().fg(COLOR_TITLE))).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_BORDER));

    f.render_widget(Clear::default(), popup_area);
    f.render_widget(popup_block, popup_area);
    // TODO
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
