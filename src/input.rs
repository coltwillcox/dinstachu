use crate::app::AppState;
use crate::fs_ops::load_directory_rows;
use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::TableState;
use std::io::Result;
use std::time::Duration;

pub fn handle_input(app_state: &mut AppState) -> Result<bool> {
    if event::poll(Duration::from_millis(500))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => {
                    app_state.is_error_displayed = false;
                    app_state.is_f1_displayed = false;
                }
                KeyCode::F(1) => app_state.is_f1_displayed = !app_state.is_f1_displayed,
                KeyCode::F(10) | KeyCode::Char('q') => return Ok(false), // Temp 'q' debug
                KeyCode::Tab => handle_tab_switching(app_state),
                KeyCode::Down => handle_move_selection(app_state, |state, len| {
                    state.select(state.selected().map_or(Some(0), |i| Some(if i >= len - 1 { 0 } else { i + 1 })));
                }),
                KeyCode::Up => handle_move_selection(app_state, |state, len| {
                    state.select(state.selected().map_or(Some(len.saturating_sub(1)), |i| Some(if i == 0 { len - 1 } else { i - 1 })));
                }),
                KeyCode::PageDown => {
                    let page_size = app_state.page_size as usize;
                    handle_move_selection(app_state, |state, len| {
                        state.select(state.selected().map(|selected| (selected + page_size).min(len.saturating_sub(1))));
                    })
                }
                KeyCode::PageUp => {
                    let page_size = app_state.page_size as usize;
                    handle_move_selection(app_state, |state, _len| {
                        state.select(state.selected().map(|selected| selected.saturating_sub(page_size)));
                    })
                }
                KeyCode::Home => handle_move_selection(app_state, |state, _len| {
                    state.select(Some(0));
                }),
                KeyCode::End => handle_move_selection(app_state, |state, len| {
                    state.select(Some(len.saturating_sub(1)));
                }),
                KeyCode::Backspace => handle_navigate_up(app_state),
                KeyCode::Enter => handle_enter_directory(app_state),
                _ => {}
            }
        }
    }
    Ok(true)
}

fn handle_tab_switching(app_state: &mut AppState<'_>) {
	if app_state.is_error_displayed || app_state.is_f1_displayed {
		return;
	}
	app_state.is_left_active = !app_state.is_left_active
}

fn handle_move_selection(app_state: &mut AppState, move_fn: impl Fn(&mut TableState, usize)) {
	if app_state.is_error_displayed || app_state.is_f1_displayed {
		return;
	}
    let (state, len) = if app_state.is_left_active {
        (&mut app_state.state_left, app_state.rows_left.len())
    } else {
        (&mut app_state.state_right, app_state.rows_right.len())
    };
    move_fn(state, len);
}

fn handle_navigate_up(app_state: &mut AppState) {
    handle_panel_operation(app_state, navigate_up_panel)
}

fn handle_enter_directory(app_state: &mut AppState) {
    handle_panel_operation(app_state, enter_directory_panel)
}

fn handle_panel_operation(app_state: &mut AppState, operation: impl FnOnce(&mut AppState)) {
	if app_state.is_error_displayed || app_state.is_f1_displayed {
		return;
	}
    operation(app_state);
}

fn navigate_up_panel(app_state: &mut AppState) {
    let dir = if app_state.is_left_active { &mut app_state.dir_left } else { &mut app_state.dir_right };
    let rows = if app_state.is_left_active { &mut app_state.rows_left } else { &mut app_state.rows_right };
    let children = if app_state.is_left_active { &mut app_state.children_left } else { &mut app_state.children_right };
    let state = if app_state.is_left_active { &mut app_state.state_left } else { &mut app_state.state_right };

	if let Some(parent) = dir.parent() {
        let name_current = dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        let dir_new = parent.to_path_buf();
        let result = load_directory_rows(&dir_new);
        match result {
            Ok((rows_new, children_new)) => {
                *dir = dir_new;
                *rows = rows_new;
                *children = children_new;
                let selected_new = children.iter().position(|item| item.name == name_current).unwrap_or(0);
                state.select(Some(selected_new));
            }
            Err(e) => {
                app_state.is_error_displayed = true;
                app_state.error_message = e.to_string();
            }
        }
    }
}

fn enter_directory_panel(app_state: &mut AppState) {
    let dir = if app_state.is_left_active { &mut app_state.dir_left } else { &mut app_state.dir_right };
    let rows = if app_state.is_left_active { &mut app_state.rows_left } else { &mut app_state.rows_right };
    let children = if app_state.is_left_active { &mut app_state.children_left } else { &mut app_state.children_right };
    let state = if app_state.is_left_active { &mut app_state.state_left } else { &mut app_state.state_right };

    if let Some(selected_index) = state.selected() {
        if let Some(item) = children.get(selected_index).cloned() {
            if item.name == ".." {
                if let Some(parent) = dir.parent() {
                    let name_current = dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                    let dir_new = parent.to_path_buf();
                    let result = load_directory_rows(&dir_new);
                    match result {
                        Ok((rows_new, children_new)) => {
                            *dir = dir_new;
                            *rows = rows_new;
                            *children = children_new;
                            let mut selected_new: usize = 0;
                            for (index, item) in children.iter_mut().enumerate() {
                                if item.name == name_current {
                                    selected_new = index;
                                }
                            }
                            state.select(Some(selected_new));
                        }
                        Err(e) => {
                            app_state.is_error_displayed = true;
                            app_state.error_message = e.to_string();
                        }
                    }
                }
            } else if item.is_dir {
                let mut dir_new = dir.clone();
                dir_new.push(item.name.clone());
                let result = load_directory_rows(&dir_new);
                match result {
                    Ok((rows_new, children_new)) => {
                        *dir = dir_new;
                        *rows = rows_new;
                        *children = children_new;
                        state.select(Some(0));
                    }
                    Err(e) => {
                        app_state.is_error_displayed = true;
                        app_state.error_message = e.to_string();
                    }
                }
            }
        }
    }
}
