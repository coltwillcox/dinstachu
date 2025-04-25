use crate::fs_ops::load_directory_rows;
use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::{Row, TableState};
use std::io::Result;
use std::path::PathBuf;
use std::time::Duration;

pub fn handle_input(
    left_dir: &mut PathBuf,
    right_dir: &mut PathBuf,
    state_left: &mut TableState,
    state_right: &mut TableState,
    is_left: &mut bool,
    rows_left: &mut Vec<Row>,
    rows_right: &mut Vec<Row>,
) -> Result<bool> {
    if event::poll(Duration::from_millis(500))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::F(10) => return Ok(false),
                KeyCode::Tab => *is_left = !*is_left,
                KeyCode::Down => {
                    let (state, len) = if *is_left {
                        (state_left, rows_left.len())
                    } else {
                        (state_right, rows_right.len())
                    };
                    if let Some(i) = state.selected() {
                        state.select(Some(if i >= len - 1 { 0 } else { i + 1 }));
                    }
                }
                KeyCode::Up => {
                    let (state, len) = if *is_left {
                        (state_left, rows_left.len())
                    } else {
                        (state_right, rows_right.len())
                    };
                    if let Some(i) = state.selected() {
                        state.select(Some(if i == 0 { len - 1 } else { i - 1 }));
                    }
                }
                KeyCode::Backspace => {
                    if *is_left {
                        if let Some(parent) = left_dir.parent() {
                            *left_dir = parent.to_path_buf();
                            *rows_left = load_directory_rows(left_dir)?;
                            state_left.select(Some(1));
                        }
                    } else {
                        if let Some(parent) = right_dir.parent() {
                            *right_dir = parent.to_path_buf();
                            *rows_right = load_directory_rows(right_dir)?;
                            state_right.select(Some(1));
                        }
                    }
                }
                KeyCode::Enter => {
                    // TODO
                    // if let Some(parent) = left_dir.parent() {
                    // 	*left_dir = parent.to_path_buf();
                    // 	*rows_left = load_directory_rows(left_dir)?;
                    // 	state_left.select(Some(1));
                    // }
                }
                _ => {}
            }
        }
    }

    Ok(true)
}
