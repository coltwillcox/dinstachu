mod app;
mod constants;
mod fs_ops;
mod input;
mod ui;
mod utils;

use app::AppState;
use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use fs_ops::load_directory_rows;
use input::handle_input;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::env;
use std::io::{Result, stdout};
use ui::render_ui;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state: AppState = AppState::new();

    match env::current_dir() {
        Ok(path) => {
            app_state.dir_left = path.clone();
            app_state.dir_right = path.clone();
        }
        Err(e) => {
            eprintln!("Failed to get current directory: {}", e);
        }
    }

    // TODO Check for error
    (app_state.rows_left, app_state.children_left) = load_directory_rows(&app_state.dir_left)?;
    (app_state.rows_right, app_state.children_right) = load_directory_rows(&app_state.dir_right)?;

    loop {
        render_ui(&mut terminal, &mut app_state);
        if !handle_input(&mut app_state)? {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
