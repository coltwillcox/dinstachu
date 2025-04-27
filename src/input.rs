use crate::fs_ops::{Item, load_directory_rows};
use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::{Row, TableState};
use std::io::Result;
use std::path::PathBuf;
use std::time::Duration;

pub fn handle_input(
    dir_left: &mut PathBuf,
    dir_right: &mut PathBuf,
    state_left: &mut TableState,
    state_right: &mut TableState,
    is_left: &mut bool,
    rows_left: &mut Vec<Row>,
    rows_right: &mut Vec<Row>,
    children_left: &mut Vec<Item>,
    children_right: &mut Vec<Item>,
    page_size: u16,
) -> Result<bool> {
    if event::poll(Duration::from_millis(500))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::F(10) => return Ok(false),
                KeyCode::Tab => *is_left = !*is_left,
                KeyCode::Down => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, len| {
                    if let Some(i) = state.selected() {
                        state.select(Some(if i >= len - 1 { 0 } else { i + 1 }));
                    }
                }),
                KeyCode::Up => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, len| {
                    if let Some(i) = state.selected() {
                        state.select(Some(if i == 0 { len - 1 } else { i - 1 }));
                    }
                }),
                KeyCode::PageDown => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, len| {
                    if let Some(selected) = state.selected() {
                        let next = (selected + page_size as usize).clamp(0, len);
                        state.select(Some(next));
                    }
                }),
                KeyCode::PageUp => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, len| {
                    if let Some(selected) = state.selected() {
                        let next = selected.saturating_sub(page_size as usize).clamp(0, len.saturating_sub(1));
                        state.select(Some(next));
                    }
                }),
                KeyCode::Home => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, _len| {
                    state.select(Some(0));
                }),
                KeyCode::End => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, len| {
                    state.select(Some(len - 1));
                }),
                KeyCode::Backspace => handle_navigate_up(is_left, dir_left, rows_left, children_left, state_left, dir_right, rows_right, children_right, state_right)?,
                KeyCode::Enter => handle_enter_directory(is_left, dir_left, rows_left, children_left, state_left, dir_right, rows_right, children_right, state_right)?,
                _ => {}
            }
        }
    }
    Ok(true)
}

fn handle_move_selection(is_left: &mut bool, state_left: &mut TableState, len_left: usize, state_right: &mut TableState, len_right: usize, move_fn: impl Fn(&mut TableState, usize)) {
    let (state, len) = if *is_left { (state_left, len_left) } else { (state_right, len_right) };
    move_fn(state, len);
}

fn handle_navigate_up(
    is_left: &mut bool,
    dir_left: &mut PathBuf,
    rows_left: &mut Vec<Row>,
    children_left: &mut Vec<Item>,
    state_left: &mut TableState,
    dir_right: &mut PathBuf,
    rows_right: &mut Vec<Row>,
    children_right: &mut Vec<Item>,
    state_right: &mut TableState,
) -> Result<()> {
    if *is_left {
        if let Some(parent) = dir_left.parent() {
            let name_current = dir_left.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            *dir_left = parent.to_path_buf();
            let (rows_new, children_new) = load_directory_rows(dir_left)?;
            *rows_left = rows_new;
            *children_left = children_new;
            let mut selected_new: usize = 0;
            for (index, item) in children_left.iter_mut().enumerate() {
                if (item.name == name_current) {
                    selected_new = index;
                }
            }
            state_left.select(Some(selected_new));
        }
    } else {
        if let Some(parent) = dir_right.parent() {
            let name_current = dir_right.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            *dir_right = parent.to_path_buf();
            let (rows_new, children_new) = load_directory_rows(dir_right)?;
            *rows_right = rows_new;
            *children_right = children_new;
            let mut selected_new: usize = 0;
            for (index, item) in children_right.iter_mut().enumerate() {
                if (item.name == name_current) {
                    selected_new = index;
                }
            }
            state_right.select(Some(selected_new));
        }
    }
    Ok(())
}

fn handle_enter_directory(
    is_left: &mut bool,
    dir_left: &mut PathBuf,
    rows_left: &mut Vec<Row>,
    children_left: &mut Vec<Item>,
    state_left: &mut TableState,
    dir_right: &mut PathBuf,
    rows_right: &mut Vec<Row>,
    children_right: &mut Vec<Item>,
    state_right: &mut TableState,
) -> Result<()> {
    if *is_left {
        if let Some(selected_index) = state_left.selected() {
            if let Some(item) = children_left.get(selected_index).cloned() {
                navigate_or_load(dir_left, rows_left, children_left, state_left, &item)?;
            }
        }
    } else {
        if let Some(selected_index) = state_right.selected() {
            if let Some(item) = children_right.get(selected_index).cloned() {
                navigate_or_load(dir_right, rows_right, children_right, state_right, &item)?;
            }
        }
    }
    Ok(())
}

fn navigate_or_load(current_dir: &mut PathBuf, rows: &mut Vec<Row>, children: &mut Vec<Item>, state: &mut TableState, item: &Item) -> Result<()> {
    if item.name == ".." {
        if let Some(parent) = current_dir.parent() {
            let name_current = current_dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            *current_dir = parent.to_path_buf();
            let (new_rows, new_children) = load_directory_rows(current_dir)?;
            *rows = new_rows;
            *children = new_children;
            let mut selected_new: usize = 0;
            for (index, item) in children.iter_mut().enumerate() {
                if (item.name == name_current) {
                    selected_new = index;
                }
            }
            state.select(Some(selected_new));
        }
    } else if item.is_dir {
        current_dir.push(item.name.clone());
        let (new_rows, new_children) = load_directory_rows(current_dir)?;
        *rows = new_rows;
        *children = new_children;
        state.select(Some(0));
    }
    Ok(())
}
