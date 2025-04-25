use ratatui::style::Color;

pub const TITLE: &str = "Dinstachu";
pub const VERSION: &str = "0.0.1";

pub const COLOR_BORDER: Color = Color::Magenta;
pub const COLOR_SELECTED_BACKGROUND_INACTIVE: Color = Color::DarkGray;
pub const COLOR_SELECTED_BACKGROUND: Color = Color::LightMagenta;
pub const COLOR_SELECTED_FOREGROUND: Color = Color::Black;
pub const COLOR_DIRECTORY_FIX: Color = Color::Yellow;
pub const COLOR_DIRECTORY: Color = Color::LightYellow;
pub const COLOR_FILE: Color = Color::Gray;

pub const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
