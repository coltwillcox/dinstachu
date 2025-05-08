use ratatui::widgets::{Row, TableState};
use std::path::PathBuf;

pub struct AppState<'a> {
    pub is_error_displayed: bool,
    pub is_f1_displayed: bool,
    pub is_left_active: bool,
    pub dir_left: PathBuf,
    pub dir_right: PathBuf,
    pub page_size: u16,
    pub state_left: TableState,
    pub state_right: TableState,
    pub rows_left: Vec<Row<'a>>,
    pub rows_right: Vec<Row<'a>>,
    pub children_left: Vec<Item>,
    pub children_right: Vec<Item>,
    pub error_message: String,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub extension: String,
    pub is_dir: bool,
    pub size: String,
}

// TODO Get root path
impl AppState<'_> {
    pub fn new() -> Self {
        let mut state_left = TableState::default();
        state_left.select(Some(1));
        let mut state_right = TableState::default();
        state_right.select(Some(1));

        Self {
            is_error_displayed: false,
            is_f1_displayed: false,
            is_left_active: true,
            dir_left: PathBuf::from("/"),
            dir_right: PathBuf::from("/"),
            page_size: 0,
            state_left,
            state_right,
            rows_left: Vec::<Row>::new(),
            rows_right: Vec::<Row>::new(),
            children_left: Vec::<Item>::new(),
            children_right: Vec::<Item>::new(),
            error_message: String::new(),
        }
    }
}
