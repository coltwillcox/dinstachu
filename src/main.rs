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
use std::env;
use std::io::{Result, stdout};
use std::path::PathBuf;
use ui::render_ui;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut dir_left = PathBuf::from("/");
    let mut dir_right = PathBuf::from("/");

    match env::current_dir() {
        Ok(path) => {
            dir_left = path.clone();
            dir_right = path.clone();
        }
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
        }
    }

    let (mut rows_left, mut children_left) = load_directory_rows(&dir_left.clone())?;
    let (mut rows_right, mut children_right) = load_directory_rows(&dir_right.clone())?;

    let mut state_left = TableState::default();
    let mut state_right = TableState::default();

    state_left.select(Some(1));
    state_right.select(Some(1));

    let mut is_left = true;
    let mut is_f1_displayed = false;
    let mut page_size: u16;

    loop {
        page_size = render_ui(&mut terminal, &mut dir_left, &mut dir_right, &rows_left, &rows_right, &state_left, &state_right, is_f1_displayed, is_left)?;
        if !handle_input(
            &mut dir_left,
            &mut dir_right,
            &mut state_left,
            &mut state_right,
            &mut rows_left,
            &mut rows_right,
            &mut children_left,
            &mut children_right,
            &mut is_left,
            &mut is_f1_displayed,
            page_size,
        )? {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
