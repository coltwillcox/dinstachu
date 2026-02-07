use ratatui::style::Color;

pub const TITLE: &str = "File Manager '84";
pub const VERSION: &str = "0.2.0";

pub const ICON_FOLDER: &str = " "; // Added space after icon is workaround for small icons bug in table columns
pub const ICON_FILE: &str = " ";
pub const ICON_LOGO: &str = "󱐌";

// Default synthwave color palette
pub const COLOR_BORDER: Color = Color::Rgb(116, 58, 213);                    // Violet
pub const COLOR_COLUMNS: Color = Color::Rgb(0, 255, 255);                    // Cyan
pub const COLOR_DIRECTORY: Color = Color::Rgb(255, 0, 255);                  // Magenta
pub const COLOR_DIRECTORY_FIX: Color = Color::Rgb(255, 0, 255);              // Magenta
pub const COLOR_FILE: Color = Color::Rgb(114, 137, 218);                     // Soft purple/blue
pub const COLOR_RENAME_BACKGROUND: Color = Color::Rgb(255, 0, 128);          // Hot pink
pub const COLOR_SELECTED_BACKGROUND: Color = Color::Rgb(148, 0, 211);        // Purple
pub const COLOR_SELECTED_BACKGROUND_INACTIVE: Color = Color::Rgb(45, 0, 75); // Dark purple
pub const COLOR_SELECTED_FOREGROUND: Color = Color::Rgb(0, 255, 255);        // Cyan
pub const COLOR_TITLE: Color = Color::Rgb(242, 34, 255);                     // Purple

pub const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
