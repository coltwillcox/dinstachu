use ratatui::style::Color;

pub const TITLE: &str = "File Manager '84";
pub const VERSION: &str = "0.0.1";

pub const ICON_FOLDER: &str = " "; // Added space after icon is workaround for small icons bug in table columns
pub const ICON_FILE: &str = " ";
pub const ICON_LOGO: &str = "󱐌";

pub const COLOR_BORDER: Color = Color::Red;
pub const COLOR_COLUMNS: Color = Color::Rgb(248, 160, 85);
pub const COLOR_DIRECTORY: Color = Color::Rgb(255, 230, 45);
pub const COLOR_DIRECTORY_FIX: Color = Color::Rgb(255, 230, 45);
pub const COLOR_FILE: Color = Color::Gray;
pub const COLOR_RENAME_BACKGROUND: Color = Color::Rgb(255, 230, 45);
pub const COLOR_SELECTED_BACKGROUND: Color = Color::Rgb(248, 160, 85);
pub const COLOR_SELECTED_BACKGROUND_INACTIVE: Color = Color::DarkGray;
pub const COLOR_SELECTED_FOREGROUND: Color = Color::Black;
pub const COLOR_TITLE: Color = Color::Rgb(244, 220, 38);

pub const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
