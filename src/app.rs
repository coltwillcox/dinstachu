use std::path::PathBuf;

pub struct AppState {
    pub is_f1_displayed: bool,
    pub is_left_active: bool,
	pub dir_left: PathBuf,
    pub dir_right: PathBuf,
	pub page_size: u16,
}

// TODO Get root path
impl AppState {
    pub fn new() -> Self {
        Self { is_f1_displayed: false, is_left_active: true, dir_left: PathBuf::from("/"), dir_right: PathBuf::from("/"), page_size: 0 }
    }
}
