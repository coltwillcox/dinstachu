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
                    state.select(state.selected().map_or(Some(0), |i| Some(if i >= len - 1 { 0 } else { i + 1 })));
                }),
                KeyCode::Up => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, len| {
                    state.select(state.selected().map_or(Some(len.saturating_sub(1)), |i| Some(if i == 0 { len - 1 } else { i - 1 })));
                }),
                KeyCode::PageDown => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, len| {
                    state.select(state.selected().map(|selected| (selected + page_size as usize).min(len.saturating_sub(1))));
                }),
                KeyCode::PageUp => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, _len| {
                    state.select(state.selected().map(|selected| selected.saturating_sub(page_size as usize)));
                }),
                KeyCode::Home => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, _len| {
                    state.select(Some(0));
                }),
                KeyCode::End => handle_move_selection(is_left, state_left, rows_left.len(), state_right, rows_right.len(), |state, len| {
                    state.select(Some(len.saturating_sub(1)));
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
    handle_panel_operation(*is_left, dir_left, rows_left, children_left, state_left, dir_right, rows_right, children_right, state_right, navigate_up_panel)
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
    handle_panel_operation(*is_left, dir_left, rows_left, children_left, state_left, dir_right, rows_right, children_right, state_right, enter_directory_panel)
}

fn handle_panel_operation<T>(
    is_left: bool,
    dir_left: &mut PathBuf,
    rows_left: &mut Vec<Row>,
    children_left: &mut Vec<T>,
    state_left: &mut TableState,
    dir_right: &mut PathBuf,
    rows_right: &mut Vec<Row>,
    children_right: &mut Vec<T>,
    state_right: &mut TableState,
    operation: impl FnOnce(&mut PathBuf, &mut Vec<Row>, &mut Vec<T>, &mut TableState) -> Result<()>,
) -> Result<()> {
    if is_left {
        operation(dir_left, rows_left, children_left, state_left)?;
    } else {
        operation(dir_right, rows_right, children_right, state_right)?;
    }
    Ok(())
}

fn navigate_up_panel(dir: &mut PathBuf, rows: &mut Vec<Row>, children: &mut Vec<Item>, state: &mut TableState) -> Result<()> {
    if let Some(parent) = dir.parent() {
        let name_current = dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        *dir = parent.to_path_buf();
        let (rows_new, children_new) = load_directory_rows(dir)?;
        *rows = rows_new;
        *children = children_new;
        let selected_new = children.iter().position(|item| item.name == name_current).unwrap_or(0);
        state.select(Some(selected_new));
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
                if item.name == name_current {
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

fn enter_directory_panel(dir: &mut PathBuf, rows: &mut Vec<Row>, children: &mut Vec<Item>, state: &mut TableState) -> Result<()> {
    if let Some(selected_index) = state.selected() {
        if let Some(item) = children.get(selected_index).cloned() {
            navigate_or_load(dir, rows, children, state, &item)?;
        }
    }
    Ok(())
}
