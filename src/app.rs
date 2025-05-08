pub struct AppState {
    pub is_f1_displayed: bool,
    pub is_left_active: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self { is_f1_displayed: false, is_left_active: true }
    }
}
