use crate::app::{AppState, Item};
use crate::fs_ops::{load_directory_rows, rename_path};
use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::TableState;
use std::io::Result;
use std::path::PathBuf;
use std::time::Duration;

pub fn handle_input(app_state: &mut AppState) -> Result<bool> {
    if event::poll(Duration::from_millis(500))? {
        if let Event::Key(key) = event::read()? {
            if app_state.is_f2_displayed {
                match key.code {
                    KeyCode::Esc => handle_esc(app_state),
                    KeyCode::F(2) => toggle_rename(app_state),
                    KeyCode::F(10) => return Ok(false), // Temp 'q' debug
                    KeyCode::Enter => handle_rename(app_state),
                    KeyCode::Char(to_insert) => app_state.enter_char(to_insert),
                    KeyCode::Backspace => app_state.delete_char(),
                    KeyCode::Left => app_state.move_cursor_left(),
                    KeyCode::Right => app_state.move_cursor_right(),
                    _ => {}
                }
            } else {
                match key.code {
                    KeyCode::Esc => handle_esc(app_state),
                    KeyCode::F(1) => app_state.is_f1_displayed = !app_state.is_f1_displayed,
                    KeyCode::F(2) => toggle_rename(app_state),
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
    }
    Ok(true)
}

fn toggle_rename(app_state: &mut AppState) {
    app_state.is_f2_displayed = !app_state.is_f2_displayed;
    if app_state.is_f2_displayed {
        let selected_index = if app_state.is_left_active { app_state.state_left.selected().unwrap() } else { app_state.state_right.selected().unwrap() };
        let selected_item = if app_state.is_left_active {
            app_state.children_left[selected_index].clone()
        } else {
            app_state.children_right[selected_index].clone()
        };
        app_state.rename_input = selected_item.name_full;
        app_state.move_cursor_end();
    } else {
        app_state.reset_rename();
    }
}

fn handle_rename(app_state: &mut AppState) {
    let parent_path = if app_state.is_left_active { &app_state.dir_left } else { &app_state.dir_right };
    let children = if app_state.is_left_active { &app_state.children_left } else { &app_state.children_right };
    let state = if app_state.is_left_active { &app_state.state_left } else { &app_state.state_right };
    let selected_item = state.selected().and_then(|index| children.get(index).cloned());

    if let Some(item) = &selected_item {
        let mut original_path = parent_path.clone();
        original_path.push(item.name_full.clone());
        let mut new_path = parent_path.clone();
        new_path.push(app_state.rename_input.clone());

        match rename_path(original_path, new_path) {
            // TODO Refresh list
            Ok(_) => println!("Successfully renamed!"),
            Err(e) => app_state.display_error(e.to_string()),
        }

        app_state.reset_rename();
    }
}

fn handle_esc(app_state: &mut AppState) {
    app_state.reset_error();
    app_state.is_f1_displayed = false;
    app_state.reset_rename();
}

fn handle_tab_switching(app_state: &mut AppState) {
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
        (&mut app_state.state_left, app_state.children_left.len())
    } else {
        (&mut app_state.state_right, app_state.children_right.len())
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
    let dir_new: std::path::PathBuf;
    let name_current: String;

    {
        let dir = if app_state.is_left_active { &mut app_state.dir_left } else { &mut app_state.dir_right };
        name_current = dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        if let Some(parent) = dir.parent() {
            dir_new = parent.to_path_buf();
        } else {
            return;
        }
    }

    let result = load_directory_rows(&app_state, &dir_new);
    match result {
        Ok(children_new) => {
            let dir = if app_state.is_left_active { &mut app_state.dir_left } else { &mut app_state.dir_right };
            let children = if app_state.is_left_active { &mut app_state.children_left } else { &mut app_state.children_right };
            let state = if app_state.is_left_active { &mut app_state.state_left } else { &mut app_state.state_right };

            *dir = dir_new;
            *children = children_new;
            let selected_new = children.iter().position(|item| item.name == name_current).unwrap_or(0);
            state.select(Some(selected_new));
        }
        Err(e) => app_state.display_error(e.to_string()),
    }
}

fn enter_directory_panel(app_state: &mut AppState) {
    let selected_item: Option<Item>;
    let mut parent_dir_new: Option<std::path::PathBuf>;
    let current_dir_name: Option<String>;
    let mut enter_subdir: Option<std::path::PathBuf>;

    {
        let state = if app_state.is_left_active { &app_state.state_left } else { &app_state.state_right };
        let children = if app_state.is_left_active { &app_state.children_left } else { &app_state.children_right };
        let dir = if app_state.is_left_active { &app_state.dir_left } else { &app_state.dir_right };

        selected_item = state.selected().and_then(|index| children.get(index).cloned());
        parent_dir_new = None;
        current_dir_name = dir.file_name().map(|n| n.to_string_lossy().to_string());
        enter_subdir = None;

        if let Some(item) = &selected_item {
            if item.name == ".." {
                if let Some(parent) = dir.parent() {
                    parent_dir_new = Some(parent.to_path_buf());
                }
            } else if item.is_dir {
                let mut dir_new = dir.clone();
                dir_new.push(item.name.clone());
                enter_subdir = Some(dir_new);
            }
        }
    }

    if let Some(dir_new) = parent_dir_new {
        let result = load_directory_rows(&app_state, &dir_new);
        match result {
            Ok(children_new) => {
                let dir = if app_state.is_left_active { &mut app_state.dir_left } else { &mut app_state.dir_right };
                let children_mut = if app_state.is_left_active { &mut app_state.children_left } else { &mut app_state.children_right };
                let state = if app_state.is_left_active { &mut app_state.state_left } else { &mut app_state.state_right };

                *dir = dir_new;
                *children_mut = children_new;
                let selected_new = children_mut.iter().position(|item| Some(&item.name) == current_dir_name.as_ref()).unwrap_or(0);
                state.select(Some(selected_new));
            }
            Err(e) => app_state.display_error(e.to_string()),
        }
    } else if let Some(dir_new) = enter_subdir {
        let result = load_directory_rows(&app_state, &dir_new);
        match result {
            Ok(children_new) => {
                let dir = if app_state.is_left_active { &mut app_state.dir_left } else { &mut app_state.dir_right };
                let children = if app_state.is_left_active { &mut app_state.children_left } else { &mut app_state.children_right };
                let state = if app_state.is_left_active { &mut app_state.state_left } else { &mut app_state.state_right };

                *dir = dir_new;
                *children = children_new;
                state.select(Some(0));
            }
            Err(e) => app_state.display_error(e.to_string()),
        }
    }
}
