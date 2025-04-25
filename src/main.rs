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
use ui::render_ui;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (rows_left, rows_right) = (load_directory_rows("/home/colt")?, load_directory_rows("/home/colt/.config")?);
    let (mut state_left, mut state_right) = (TableState::default(), TableState::default());
    state_left.select(Some(1));
    state_right.select(Some(1));
	let mut is_left = true;

    loop {
        render_ui(&mut terminal, &rows_left, &rows_right, &state_left, &state_right, is_left)?;
        if !handle_input(&mut state_left, &mut state_right, &mut is_left, rows_left.len(), rows_right.len())? {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}
