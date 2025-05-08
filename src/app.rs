use ratatui::widgets::TableState;
use std::path::PathBuf;

pub struct AppState {
    pub is_f1_displayed: bool,
    pub is_left_active: bool,
    pub dir_left: PathBuf,
    pub dir_right: PathBuf,
    pub page_size: u16,
    pub state_left: TableState,
    pub state_right: TableState,
}

// TODO Get root path
impl AppState {
    pub fn new() -> Self {
		let mut state_left = TableState::default();
        state_left.select(Some(1));
		let mut state_right = TableState::default();
		state_right.select(Some(1));

        Self {
            is_f1_displayed: false,
            is_left_active: true,
            dir_left: PathBuf::from("/"),
            dir_right: PathBuf::from("/"),
            page_size: 0,
			state_left: state_left,
			state_right: state_right,
        }
    }
}
