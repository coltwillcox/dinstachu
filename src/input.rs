use crossterm::event::{self, Event, KeyCode};
use ratatui::widgets::TableState;
use std::io::Result;
use std::time::Duration;

pub fn handle_input(state_left: &mut TableState, state_right: &mut TableState, is_left: &mut bool, len_left: usize, len_right: usize) -> Result<bool> {
    if event::poll(Duration::from_millis(500))? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::F(10) => return Ok(false),
                KeyCode::Tab => *is_left = !*is_left,
                KeyCode::Down => {
                    let (state, len) = if *is_left { (state_left, len_left) } else { (state_right, len_right) };
                    if let Some(i) = state.selected() {
                        state.select(Some(if i >= len - 1 { 0 } else { i + 1 }));
                    }
                }
                KeyCode::Up => {
                    let (state, len) = if *is_left { (state_left, len_left) } else { (state_right, len_right) };
                    if let Some(i) = state.selected() {
                        state.select(Some(if i == 0 { len - 1 } else { i - 1 }));
                    }
                }
                _ => {}
            }
        }
    }

    Ok(true)
}
