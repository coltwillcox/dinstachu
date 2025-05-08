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
use ratatui::{Terminal, backend::CrosstermBackend, widgets::TableState};
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
    let (mut rows_left, mut children_left) = load_directory_rows(&app_state.dir_left)?;
    let (mut rows_right, mut children_right) = load_directory_rows(&app_state.dir_right)?;
    
    loop {
        render_ui(&mut terminal, &rows_left, &rows_right, &mut app_state);
        if !handle_input(&mut rows_left, &mut rows_right, &mut children_left, &mut children_right, &mut app_state)? {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
