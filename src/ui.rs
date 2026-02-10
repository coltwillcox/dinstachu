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
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, TableState},
};
use std::path::PathBuf;

pub fn render_ui<B: Backend>(terminal: &mut Terminal<B>, app_state: &mut AppState) {
    // Update cached clock
    let current_time = Local::now().format(" %H:%M:%S ").to_string();
    if app_state.cached_clock != current_time {
        app_state.cached_clock = current_time;
    }

    let _ = terminal.draw(|f| {
        let area = f.area();

        // Guard against terminal too small to render
        if area.height < 10 || area.width < 30 {
            let msg = Paragraph::new("Terminal too small")
                .alignment(Alignment::Center)
                .style(Style::default().fg(COLOR_TITLE));
            let y = area.height / 2;
            if y < area.height {
                f.render_widget(msg, Rect::new(area.x, area.y + y, area.width, 1));
            }
            return;
        }

        let chunks_main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Length(1), Constraint::Percentage(100), Constraint::Length(1), Constraint::Length(3)])
            .split(area);

        render_top_panel(f, chunks_main[0], &app_state.cached_clock);
        render_path_bar(f, chunks_main[1], &app_state.dir_left, &app_state.dir_right, area.width, app_state.is_left_active);
        if app_state.is_f3_displayed {
            app_state.viewer_viewport_height = render_viewer(f, chunks_main[2], app_state);
        } else if app_state.is_f4_displayed {
            app_state.editor_viewport_height = render_editor(f, chunks_main[2], app_state);
        } else {
            app_state.page_size = render_file_tables(f, chunks_main[2], app_state);
        }
        render_bottom_panel(f, chunks_main[3], app_state);
        render_fkey_bar(f, chunks_main[4]);

        if app_state.is_error_displayed {
            render_error_popup(f, area, app_state);
        } else if app_state.is_editor_save_prompt {
            render_editor_save_popup(f, area);
        } else if app_state.is_f1_displayed {
            render_help_popup(f, area);
        } else if app_state.is_f5_displayed {
            render_copy_popup(f, area, app_state);
        } else if app_state.is_f6_displayed {
            render_move_popup(f, area, app_state);
        } else if app_state.is_f7_displayed {
            render_create_popup(f, area, app_state);
        } else if app_state.is_f8_displayed {
            render_delete_popup(f, area, app_state);
        }
    });
}

fn render_top_panel(f: &mut ratatui::Frame<'_>, area: Rect, cached_clock: &str) {
    let logo = Span::styled(format!(" {} ", ICON_LOGO), Style::default().fg(COLOR_TITLE));
    let title = Span::styled(format!(" {} v{} ", TITLE, VERSION), Style::default().fg(COLOR_TITLE));
    let clock = Span::styled(cached_clock, Style::default().fg(COLOR_TITLE));

    let block_top = Block::default()
        .title_top(Line::from(logo).left_aligned())
        .title_top(Line::from(title).centered())
        .title_top(Line::from(clock).right_aligned())
        .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
        .border_style(Style::default().fg(COLOR_BORDER));

    f.render_widget(block_top, area);
}

fn render_path_bar(f: &mut ratatui::Frame<'_>, area: Rect, dir_left: &PathBuf, dir_right: &PathBuf, total_width: u16, is_left_active: bool) {
    let length_left = ((total_width as usize).saturating_sub(3)) / 2;
    let length_right = ((total_width as usize).saturating_sub(2)) / 2;

    let path_left = limit_path_string(dir_left, length_left.saturating_sub(8));
    let path_right = limit_path_string(dir_right, length_right.saturating_sub(8));

    let color_left = if is_left_active { COLOR_DIRECTORY } else { COLOR_DIRECTORY_DARK };
    let color_right = if is_left_active { COLOR_DIRECTORY_DARK } else { COLOR_DIRECTORY };

    let border_line = vec![
        Span::styled(format!("{}", "├──"), Style::default().fg(COLOR_BORDER)),
        Span::styled(format!(" {} ", path_left), Style::default().fg(color_left)),
        Span::styled(format!("{}{}", "─".repeat(length_left.saturating_sub(path_left.len().saturating_add(5))), "─┬──"), Style::default().fg(COLOR_BORDER)),
        Span::styled(format!(" {} ", path_right), Style::default().fg(color_right)),
        Span::styled(format!("{}{}", "─".repeat(length_right.saturating_sub(path_right.len().saturating_add(5))), "─┤"), Style::default().fg(COLOR_BORDER)),
    ];

    f.render_widget(Paragraph::new(Line::from(border_line)), area);
}

fn render_file_tables(f: &mut ratatui::Frame<'_>, chunk: Rect, app_state: &mut AppState) -> u16 {
    let chunks = Layout::default().direction(Direction::Horizontal).constraints([Constraint::Percentage(50), Constraint::Length(1), Constraint::Percentage(50)]).split(chunk);

    let widths = [Constraint::Length(2), Constraint::Percentage(50), Constraint::Length(1), Constraint::Percentage(10), Constraint::Length(1), Constraint::Percentage(15), Constraint::Length(1), Constraint::Length(15)];

    let is_f2_displayed = app_state.is_f2_displayed;
    let table_style = |active: bool| {
        Style::default()
            .bg(if active {
                if is_f2_displayed { COLOR_RENAME_BACKGROUND } else { COLOR_SELECTED_BACKGROUND }
            } else {
                COLOR_SELECTED_BACKGROUND_INACTIVE
            })
            .fg(COLOR_SELECTED_FOREGROUND)
            .add_modifier(Modifier::BOLD)
    };

    // Viewport height (subtract 1 for header row)
    let viewport_height = chunks[0].height.saturating_sub(1) as usize;

    // Get rename input once if in rename mode
    let rename_input = if app_state.is_f2_displayed {
        Some(app_state.get_rename_input())
    } else {
        None
    };

    // Build only visible rows for left panel
    let (rows_left, offset_left) = build_viewport_rows(app_state, true, viewport_height, rename_input.as_deref());
    let mut state_left_view = TableState::default();
    state_left_view.select(app_state.state_left.selected().map(|s| s.saturating_sub(offset_left)));

    let table_left = Table::new(rows_left, widths.clone())
        .block(Block::default().borders(Borders::LEFT).border_style(Style::default().fg(COLOR_BORDER)))
        .header(make_header_row())
        .row_highlight_style(table_style(app_state.is_left_active))
        .column_spacing(1);
    f.render_stateful_widget(table_left, chunks[0], &mut state_left_view);

    // Cache the separator string based on height
    let separator_height = chunks[0].height;
    if app_state.cached_separator_height != separator_height {
        app_state.cached_separator_height = separator_height;
        app_state.cached_separator = "│\n".repeat(separator_height.saturating_sub(1) as usize) + "│";
    }
    let separator_vertical = Paragraph::new(Text::raw(&app_state.cached_separator)).style(Style::default().fg(COLOR_BORDER));
    f.render_widget(separator_vertical, chunks[1]);

    // Build only visible rows for right panel
    let (rows_right, offset_right) = build_viewport_rows(app_state, false, viewport_height, rename_input.as_deref());
    let mut state_right_view = TableState::default();
    state_right_view.select(app_state.state_right.selected().map(|s| s.saturating_sub(offset_right)));

    let table_right = Table::new(rows_right, widths)
        .block(Block::default().borders(Borders::RIGHT).border_style(Style::default().fg(COLOR_BORDER)))
        .header(make_header_row())
        .row_highlight_style(table_style(!app_state.is_left_active))
        .column_spacing(1);
    f.render_stateful_widget(table_right, chunks[2], &mut state_right_view);

    chunks[0].height
}

/// Build only the rows visible in the viewport, returns (rows, start_offset)
fn build_viewport_rows(app_state: &AppState, is_left: bool, viewport_height: usize, rename_input: Option<&str>) -> (Vec<Row<'static>>, usize) {
    let children = if is_left { &app_state.children_left } else { &app_state.children_right };
    let state = if is_left { &app_state.state_left } else { &app_state.state_right };
    let selected_set = if is_left { &app_state.selected_left } else { &app_state.selected_right };
    let current_dir = if is_left { &app_state.dir_left } else { &app_state.dir_right };
    let selected = state.selected().unwrap_or(0);
    let total = children.len();

    if total == 0 {
        return (Vec::new(), 0);
    }

    // Calculate viewport window centered on selection
    let half_view = viewport_height / 2;
    let start = if selected <= half_view {
        0
    } else if selected + half_view >= total {
        total.saturating_sub(viewport_height)
    } else {
        selected.saturating_sub(half_view)
    };
    let end = (start + viewport_height).min(total);

    let is_renaming_current_side = app_state.is_f2_displayed && (app_state.is_left_active == is_left);

    let mut rows = Vec::with_capacity(end - start);

    for index in start..end {
        let child = &children[index];
        let is_renaming_current_item = is_renaming_current_side && (index == selected);
        let is_selected = selected_set.contains(&index);

        // Keep original icon, change color if selected
        let icon = if child.is_dir { ICON_FOLDER } else { ICON_FILE };
        let file_color = color_for_extension(&child.extension);
        let icon_color = if is_selected {
            COLOR_SELECTED_MARKER
        } else if child.is_dir {
            COLOR_DIRECTORY
        } else {
            file_color
        };

        let (dir_prefix, dir_suffix) = if child.is_dir { ("[", "]") } else { ("", "") };

        let name = if is_renaming_current_item {
            rename_input.unwrap().to_string()
        } else {
            child.name.clone()
        };
        let extension = if is_renaming_current_item { String::new() } else { child.extension.clone() };

        // Use selection marker color for selected items
        let text_color = if is_selected {
            COLOR_SELECTED_MARKER
        } else if child.is_dir {
            COLOR_DIRECTORY
        } else {
            file_color
        };

        // Get size - for directories, show calculated size if available
        let size = if child.is_dir && child.name != ".." {
            let mut full_path = current_dir.clone();
            full_path.push(&child.name_full);
            if let Some(&calculated_size) = app_state.dir_sizes.get(&full_path) {
                format_size(calculated_size)
            } else {
                child.size.clone()
            }
        } else {
            child.size.clone()
        };

        rows.push(Row::new(vec![
            Cell::from(Span::styled(icon, Style::default().fg(icon_color))),
            Cell::from(Line::from(vec![
                Span::styled(dir_prefix, Style::default().fg(if is_selected { COLOR_SELECTED_MARKER } else { COLOR_DIRECTORY_FIX })),
                Span::styled(name, Style::default().fg(text_color)),
                Span::styled(dir_suffix, Style::default().fg(if is_selected { COLOR_SELECTED_MARKER } else { COLOR_DIRECTORY_FIX })),
            ])),
            Cell::from(Span::styled("│", Style::default().fg(COLOR_BORDER))),
            Cell::from(Span::styled(extension, Style::default().fg(text_color))),
            Cell::from(Span::styled("│", Style::default().fg(COLOR_BORDER))),
            Cell::from(Span::styled(size, Style::default().fg(text_color))),
            Cell::from(Span::styled("│", Style::default().fg(COLOR_BORDER))),
            Cell::from(Span::styled(child.modified.clone(), Style::default().fg(text_color))),
        ]));
    }

    (rows, start)
}

fn make_header_row() -> Row<'static> {
    Row::new(vec![
        Cell::from(Span::styled("", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("Name", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("Ext", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("Size", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("", Style::default().fg(COLOR_COLUMNS))),
        Cell::from(Span::styled("Modified", Style::default().fg(COLOR_COLUMNS))),
    ])
}

fn render_viewer(f: &mut ratatui::Frame<'_>, area: Rect, app_state: &AppState) -> usize {
    if let Some(viewer_state) = &app_state.viewer_state {
        let filename = viewer_state.file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");
        let title = format!(" View: {} ", filename);

        let border_block = Block::default()
            .title(Line::from(Span::styled(title, Style::default().fg(COLOR_TITLE))).centered())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_BORDER));

        let inner_area = border_block.inner(area);
        f.render_widget(border_block, area);

        // Calculate line number gutter width
        let line_num_width = (viewer_state.total_lines.to_string().len() as u16).max(3) + 2;

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(line_num_width), Constraint::Min(0)])
            .split(inner_area);

        let viewport_height = inner_area.height as usize;
        let start = viewer_state.scroll_offset;
        let end = (start + viewport_height).min(viewer_state.total_lines);

        // Render line numbers
        let mut line_numbers = Vec::new();
        for line_num in start..end {
            line_numbers.push(Line::from(Span::styled(
                format!(
                    "{:>width$} ",
                    line_num + 1,
                    width = line_num_width as usize - 1
                ),
                Style::default().fg(COLOR_COLUMNS),
            )));
        }
        let line_number_para = Paragraph::new(line_numbers)
            .style(Style::default().bg(ratatui::style::Color::Black));
        f.render_widget(line_number_para, chunks[0]);

        // Render content
        if viewer_state.is_binary {
            let binary_msg = Paragraph::new("Binary file detected. Press Esc to return.")
                .alignment(Alignment::Center)
                .style(Style::default().fg(COLOR_TITLE));
            f.render_widget(binary_msg, chunks[1]);
        } else {
            let content_lines: Vec<Line> = viewer_state.content_lines[start..end]
                .iter()
                .map(|line| Line::from(Span::raw(line.clone())))
                .collect();

            let content_para =
                Paragraph::new(content_lines).style(Style::default().fg(COLOR_FILE));
            f.render_widget(content_para, chunks[1]);
        }

        viewport_height
    } else {
        0
    }
}

fn render_editor(f: &mut ratatui::Frame<'_>, area: Rect, app_state: &AppState) -> usize {
    if let Some(editor_state) = &app_state.editor_state {
        let filename = editor_state.file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");
        let modified = if editor_state.modified { " [Modified]" } else { "" };
        let title = format!(" Edit: {}{} ", filename, modified);

        let border_block = Block::default()
            .title(Line::from(Span::styled(title, Style::default().fg(COLOR_TITLE))).centered())
            .borders(Borders::ALL)
            .border_style(Style::default().fg(COLOR_BORDER));

        let inner_area = border_block.inner(area);
        f.render_widget(border_block, area);

        // Calculate line number gutter width
        let total_lines = editor_state.lines.len();
        let line_num_width = (total_lines.to_string().len() as u16).max(3) + 2;

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(line_num_width), Constraint::Min(0)])
            .split(inner_area);

        let viewport_height = inner_area.height as usize;
        let start = editor_state.scroll_offset;
        let end = (start + viewport_height).min(total_lines);

        // Render line numbers
        let mut line_numbers = Vec::new();
        for line_num in start..end {
            let is_current = line_num == editor_state.cursor_line;
            let style = if is_current {
                Style::default().fg(COLOR_TITLE).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(COLOR_COLUMNS)
            };
            line_numbers.push(Line::from(Span::styled(
                format!("{:>width$} ", line_num + 1, width = line_num_width as usize - 1),
                style,
            )));
        }
        let line_number_para = Paragraph::new(line_numbers)
            .style(Style::default().bg(ratatui::style::Color::Black));
        f.render_widget(line_number_para, chunks[0]);

        // Render content with cursor and syntax highlighting
        let has_highlighting = !editor_state.highlighted_lines.is_empty();
        let mut content_lines: Vec<Line> = Vec::new();
        for (idx, line) in editor_state.lines[start..end].iter().enumerate() {
            let actual_line_idx = start + idx;
            let is_current_line = actual_line_idx == editor_state.cursor_line;

            if is_current_line {
                // Show cursor on this line (plain text with cursor block)
                let cursor_col = editor_state.cursor_col;
                let chars: Vec<char> = line.chars().collect();
                let before: String = chars[..cursor_col.min(chars.len())].iter().collect();
                let cursor_char = chars.get(cursor_col).copied().unwrap_or(' ');
                let after: String = if cursor_col < chars.len() {
                    chars[cursor_col + 1..].iter().collect()
                } else {
                    String::new()
                };

                content_lines.push(Line::from(vec![
                    Span::styled(before, Style::default().fg(COLOR_FILE)),
                    Span::styled(
                        cursor_char.to_string(),
                        Style::default().fg(COLOR_SELECTED_FOREGROUND).bg(COLOR_SELECTED_BACKGROUND),
                    ),
                    Span::styled(after, Style::default().fg(COLOR_FILE)),
                ]));
            } else if has_highlighting && actual_line_idx < editor_state.highlighted_lines.len() {
                content_lines.push(Line::from(editor_state.highlighted_lines[actual_line_idx].clone()));
            } else {
                content_lines.push(Line::from(Span::styled(
                    line.clone(),
                    Style::default().fg(COLOR_FILE),
                )));
            }
        }

        let content_para = Paragraph::new(content_lines);
        f.render_widget(content_para, chunks[1]);

        viewport_height
    } else {
        0
    }
}

fn render_bottom_panel(f: &mut ratatui::Frame<'_>, area: Rect, app_state: &AppState) {
    if app_state.is_f4_displayed {
        // Show editor status
        if let Some(editor_state) = &app_state.editor_state {
            let filename = editor_state
                .file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");
            let modified = if editor_state.modified { " [Modified]" } else { "" };
            let line = editor_state.cursor_line + 1;
            let col = editor_state.cursor_col + 1;
            let total = editor_state.lines.len();

            let status_text = format!(
                " {}{} | Ln {}, Col {} | {} lines | F2/Ctrl+S Save | Esc Exit ",
                filename, modified, line, col, total
            );

            let status_line = vec![
                Span::styled("├─", Style::default().fg(COLOR_BORDER)),
                Span::styled(
                    status_text.clone(),
                    Style::default()
                        .fg(COLOR_TITLE)
                        .bg(COLOR_SELECTED_BACKGROUND),
                ),
                Span::styled(
                    "─".repeat((area.width as usize).saturating_sub(status_text.len()).saturating_sub(3)),
                    Style::default().fg(COLOR_BORDER),
                ),
                Span::styled("┤", Style::default().fg(COLOR_BORDER)),
            ];
            f.render_widget(Paragraph::new(Line::from(status_line)), area);
        }
    } else if app_state.is_f3_displayed {
        // Show viewer status
        if let Some(viewer_state) = &app_state.viewer_state {
            let filename = viewer_state
                .file_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Unknown");
            let current_line = viewer_state.scroll_offset + 1;
            let total_lines = viewer_state.total_lines;
            let file_size = format_size(viewer_state.file_size);

            let status_text = format!(
                " {} | Line {}/{} | {} | {} ",
                filename, current_line, total_lines, file_size, viewer_state.syntax_name
            );

            let status_line = vec![
                Span::styled("├─", Style::default().fg(COLOR_BORDER)),
                Span::styled(
                    status_text.clone(),
                    Style::default()
                        .fg(COLOR_TITLE)
                        .bg(COLOR_SELECTED_BACKGROUND),
                ),
                Span::styled(
                    "─".repeat((area.width as usize).saturating_sub(status_text.len()).saturating_sub(3)),
                    Style::default().fg(COLOR_BORDER),
                ),
                Span::styled("┤", Style::default().fg(COLOR_BORDER)),
            ];
            f.render_widget(Paragraph::new(Line::from(status_line)), area);
        }
    } else if !app_state.search_input.is_empty() {
        // Show search string
        let search_text = format!("Search: {}", app_state.search_input);
        let search_line = vec![
            Span::styled("├─", Style::default().fg(COLOR_BORDER)),
            Span::styled(
                format!(" {} ", search_text),
                Style::default()
                    .fg(COLOR_TITLE)
                    .bg(COLOR_SELECTED_BACKGROUND),
            ),
            Span::styled(
                "─".repeat((area.width as usize).saturating_sub(search_text.len()).saturating_sub(5)),
                Style::default().fg(COLOR_BORDER),
            ),
            Span::styled("┤", Style::default().fg(COLOR_BORDER)),
        ];
        f.render_widget(Paragraph::new(Line::from(search_line)), area);
    } else {
        // Show separator
        let total_width = area.width;
        let separator = format!(
            "├{}┴{}┤",
            "─".repeat(((total_width as usize).saturating_sub(3)) / 2),
            "─".repeat(((total_width as usize).saturating_sub(2)) / 2)
        );
        f.render_widget(
            Paragraph::new(Text::raw(separator)).style(Style::default().fg(COLOR_BORDER)),
            area,
        );
    }
}

fn render_fkey_bar(f: &mut ratatui::Frame<'_>, area: Rect) {
    let block_bottom = Block::default()
        .title_bottom(Line::from(Span::styled(" F1 Help ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F2 Rename ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F3 View ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F4 Edit ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F5 Copy ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F6 Move ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F7 Create ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F8 Delete ", Style::default().fg(COLOR_TITLE))).centered())
        .title_bottom(Line::from(Span::styled(" F9 Terminal ", Style::default().fg(COLOR_TITLE))).centered())
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
    let help_lines = vec![
        "F1 - This help",
        "F2 - Rename folder/file",
        "F3 - View file",
        "F4 - Edit file (Ctrl+S/F2 save)",
        "F5 - Copy to other panel",
        "F6 - Move to other panel",
        "F7 - Create directory",
        "F8 - Delete folder/file",
        "F9 - Open terminal",
        "F10 - Quit",
        "Space - Select/deselect file",
        "Type to search, Esc to clear",
    ];

    // 2 border rows + 1 top padding + 1 bottom padding + content lines
    let content_height = (help_lines.len() as u16) + 4;
    let popup_height = content_height.min(area.height);
    let popup_width = (area.width * 60 / 100).max(1);
    let y = area.y + (area.height.saturating_sub(popup_height)) / 2;
    let x = area.x + (area.width.saturating_sub(popup_width)) / 2;
    let popup_area = Rect::new(x, y, popup_width, popup_height);

    let popup_block = Block::default()
        .title(Line::from(Span::styled(" Help/About ", Style::default().fg(COLOR_TITLE))).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_BORDER));

    f.render_widget(Clear::default(), popup_area);
    f.render_widget(popup_block, popup_area);

    let inner = popup_area.inner(Margin { vertical: 2, horizontal: 2 });
    let max_len = help_lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let lines: Vec<Line> = help_lines.iter()
        .map(|&text| Line::from(Span::styled(format!("{:<width$}", text, width = max_len), Style::default().fg(COLOR_TITLE))))
        .collect();
    let help_para = Paragraph::new(lines).alignment(Alignment::Center);
    f.render_widget(help_para, inner);
}

fn render_create_popup(f: &mut ratatui::Frame<'_>, area: Rect, app_state: &AppState) {
    let popup_area = centered_rect(60, 20, area);
    let popup_block = Block::default()
        .title(Line::from(Span::styled(" Create Directory ", Style::default().fg(COLOR_TITLE))).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_BORDER));

    f.render_widget(Clear::default(), popup_area);
    f.render_widget(popup_block, popup_area);

    // Show input with cursor
    let input_display = app_state.get_create_input();
    f.render_widget(
        Paragraph::new(input_display).alignment(Alignment::Center).style(Style::default().fg(COLOR_TITLE).bg(COLOR_SELECTED_BACKGROUND)),
        popup_area.inner(Margin { vertical: 3, horizontal: 2 }),
    );

    // Instructions
    f.render_widget(
        Paragraph::new("Enter - Create    Esc - Cancel").alignment(Alignment::Center).style(Style::default().fg(COLOR_COLUMNS)),
        popup_area.inner(Margin { vertical: 5, horizontal: 2 }),
    );
}

fn render_delete_popup(f: &mut ratatui::Frame<'_>, area: Rect, app_state: &AppState) {
    let popup_area = centered_rect(60, 30, area);

    let item_type = if app_state.delete_item_is_dir { "directory" } else { "file" };
    let title = format!(" Delete {} ", item_type);

    let popup_block = Block::default()
        .title(Line::from(Span::styled(title, Style::default().fg(COLOR_TITLE))).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_BORDER));

    f.render_widget(Clear::default(), popup_area);
    f.render_widget(popup_block, popup_area);

    // Message
    let message = format!("Delete \"{}\"?", app_state.delete_item_name);
    f.render_widget(Paragraph::new(message).alignment(Alignment::Center).style(Style::default().fg(COLOR_TITLE)), popup_area.inner(Margin { vertical: 3, horizontal: 2 }));

    // Instructions
    f.render_widget(
        Paragraph::new("Y / Enter - Yes    N / Esc - No").alignment(Alignment::Center).style(Style::default().fg(COLOR_COLUMNS)),
        popup_area.inner(Margin { vertical: 5, horizontal: 2 }),
    );
}

fn render_copy_popup(f: &mut ratatui::Frame<'_>, area: Rect, app_state: &AppState) {
    let popup_area = centered_rect(70, 35, area);

    let item_type = if app_state.copy_is_dir { "directory" } else { "file" };
    let title = format!(" Copy {} ", item_type);

    let popup_block = Block::default()
        .title(Line::from(Span::styled(title, Style::default().fg(COLOR_TITLE))).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_BORDER));

    f.render_widget(Clear::default(), popup_area);
    f.render_widget(popup_block, popup_area);

    // Source path
    let source_name = app_state.copy_source_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown");
    f.render_widget(
        Paragraph::new(format!("Copy \"{}\"", source_name)).alignment(Alignment::Center).style(Style::default().fg(COLOR_TITLE)),
        popup_area.inner(Margin { vertical: 2, horizontal: 2 }),
    );

    // Destination path
    let dest_display = limit_path_string(&app_state.copy_dest_path, popup_area.width as usize - 10);
    f.render_widget(
        Paragraph::new(format!("to: {}", dest_display)).alignment(Alignment::Center).style(Style::default().fg(COLOR_FILE)),
        popup_area.inner(Margin { vertical: 4, horizontal: 2 }),
    );

    // Instructions
    f.render_widget(
        Paragraph::new("Y / Enter - Yes    N / Esc - No").alignment(Alignment::Center).style(Style::default().fg(COLOR_COLUMNS)),
        popup_area.inner(Margin { vertical: 6, horizontal: 2 }),
    );
}

fn render_move_popup(f: &mut ratatui::Frame<'_>, area: Rect, app_state: &AppState) {
    let popup_area = centered_rect(70, 35, area);

    let item_type = if app_state.move_is_dir { "directory" } else { "file" };
    let title = format!(" Move {} ", item_type);

    let popup_block = Block::default()
        .title(Line::from(Span::styled(title, Style::default().fg(COLOR_TITLE))).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_BORDER));

    f.render_widget(Clear::default(), popup_area);
    f.render_widget(popup_block, popup_area);

    // Source path
    let source_name = app_state.move_source_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("Unknown");
    f.render_widget(
        Paragraph::new(format!("Move \"{}\"", source_name)).alignment(Alignment::Center).style(Style::default().fg(COLOR_TITLE)),
        popup_area.inner(Margin { vertical: 2, horizontal: 2 }),
    );

    // Destination path
    let dest_display = limit_path_string(&app_state.move_dest_path, popup_area.width as usize - 10);
    f.render_widget(
        Paragraph::new(format!("to: {}", dest_display)).alignment(Alignment::Center).style(Style::default().fg(COLOR_FILE)),
        popup_area.inner(Margin { vertical: 4, horizontal: 2 }),
    );

    // Instructions
    f.render_widget(
        Paragraph::new("Y / Enter - Yes    N / Esc - No").alignment(Alignment::Center).style(Style::default().fg(COLOR_COLUMNS)),
        popup_area.inner(Margin { vertical: 6, horizontal: 2 }),
    );
}

fn render_editor_save_popup(f: &mut ratatui::Frame<'_>, area: Rect) {
    let popup_area = centered_rect(60, 25, area);
    let popup_block = Block::default()
        .title(Line::from(Span::styled(" Unsaved Changes ", Style::default().fg(COLOR_TITLE))).centered())
        .borders(Borders::ALL)
        .style(Style::default().fg(COLOR_BORDER));

    f.render_widget(Clear::default(), popup_area);
    f.render_widget(popup_block, popup_area);

    f.render_widget(
        Paragraph::new("Save changes before closing?").alignment(Alignment::Center).style(Style::default().fg(COLOR_TITLE)),
        popup_area.inner(Margin { vertical: 3, horizontal: 2 }),
    );

    f.render_widget(
        Paragraph::new("Y - Save    N - Discard    Esc - Cancel").alignment(Alignment::Center).style(Style::default().fg(COLOR_COLUMNS)),
        popup_area.inner(Margin { vertical: 5, horizontal: 2 }),
    );
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
