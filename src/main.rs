// TODO When going back, select parent folder, not 0 index
// TODO Add icons column
// TODO Check if dir is deleted
// TODO Check PermissionDenied (eg. /root)
mod constants;
mod fs_ops;
mod input;
mod ui;
mod utils;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use fs_ops::load_directory_rows;
use input::handle_input;
use ratatui::{Terminal, backend::CrosstermBackend, widgets::TableState};
use std::io::{Result, stdout};
use std::path::PathBuf;
use ui::render_ui;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut left_dir = PathBuf::from("/home/colt");
    let mut right_dir = PathBuf::from("/home/colt/.config");

    let left_dir_clone = left_dir.clone();
    let right_dir_clone = right_dir.clone();

    let (mut rows_left, mut children_left) = load_directory_rows(&left_dir_clone)?;
    let (mut rows_right, mut children_right) = load_directory_rows(&right_dir_clone)?;

    let mut state_left = TableState::default();
    let mut state_right = TableState::default();

    state_left.select(Some(1));
    state_right.select(Some(1));

    let mut is_left = true;
    let mut page_size: u16;

    loop {
        page_size = render_ui(&mut terminal, &rows_left, &rows_right, &state_left, &state_right, is_left)?;
        if !handle_input(
            &mut left_dir,
            &mut right_dir,
            &mut state_left,
            &mut state_right,
            &mut is_left,
            &mut rows_left,
            &mut rows_right,
            &mut children_left,
            &mut children_right,
            page_size,
        )? {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
