use ratatui::style::Color;
use ratatui::text::Span;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;
use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

pub struct ViewerState {
    pub file_path: PathBuf,
    pub content_lines: Vec<String>,
    pub highlighted_lines: Vec<Vec<Span<'static>>>,
    pub scroll_offset: usize,
    pub total_lines: usize,
    pub file_size: u64,
    pub is_binary: bool,
    pub syntax_name: String,
}

pub fn is_binary_file(path: &PathBuf) -> Result<bool, Error> {
    let mut file = File::open(path)?;
    let mut buffer = [0; 512];
    let bytes_read = file.read(&mut buffer)?;

    // Check for null bytes
    if buffer[..bytes_read].contains(&0) {
        return Ok(true);
    }

    // Check UTF-8 validity
    std::str::from_utf8(&buffer[..bytes_read])
        .map(|_| false)
        .or(Ok(true))
}

pub fn load_file_content(path: &PathBuf) -> Result<ViewerState, Error> {
    // Check binary first
    if is_binary_file(path)? {
        return Ok(ViewerState {
            file_path: path.clone(),
            content_lines: vec!["Binary file detected.".to_string()],
            highlighted_lines: vec![],
            scroll_offset: 0,
            total_lines: 1,
            file_size: std::fs::metadata(path)?.len(),
            is_binary: true,
            syntax_name: "Binary".to_string(),
        });
    }

    // Load text file
    let content = std::fs::read_to_string(path)?;
    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let total_lines = lines.len().max(1); // At least 1 line for empty files
    let file_size = std::fs::metadata(path)?.len();
    let syntax_name = detect_syntax(path);

    Ok(ViewerState {
        file_path: path.clone(),
        content_lines: lines,
        highlighted_lines: vec![],
        scroll_offset: 0,
        total_lines,
        file_size,
        is_binary: false,
        syntax_name,
    })
}

pub fn detect_syntax(path: &PathBuf) -> String {
    match path.extension().and_then(|s| s.to_str()) {
        Some("rs") => "Rust".to_string(),
        Some("py") => "Python".to_string(),
        Some("js") | Some("jsx") => "JavaScript".to_string(),
        Some("ts") | Some("tsx") => "TypeScript".to_string(),
        Some("json") => "JSON".to_string(),
        Some("toml") => "TOML".to_string(),
        Some("yaml") | Some("yml") => "YAML".to_string(),
        Some("md") => "Markdown".to_string(),
        Some("sh") | Some("bash") => "Shell".to_string(),
        Some("c") | Some("h") => "C".to_string(),
        Some("cpp") | Some("hpp") => "C++".to_string(),
        Some("html") => "HTML".to_string(),
        Some("css") => "CSS".to_string(),
        _ => "Plain Text".to_string(),
    }
}

pub fn highlight_content(content: &[String], syntax: &str) -> Vec<Vec<Span<'static>>> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let syntax_def = ps
        .find_syntax_by_name(syntax)
        .or_else(|| Some(ps.find_syntax_plain_text()));

    if syntax_def.is_none() {
        // Fallback to plain text spans
        return content
            .iter()
            .map(|line| vec![Span::raw(line.clone())])
            .collect();
    }

    let syntax_def = syntax_def.unwrap();
    let mut h = HighlightLines::new(syntax_def, &ts.themes["base16-ocean.dark"]);

    let mut result = Vec::new();
    for line in content {
        let ranges = h.highlight_line(line, &ps).unwrap_or_default();
        let spans: Vec<Span<'static>> = ranges
            .into_iter()
            .map(|(style, text)| {
                let fg = syntect_to_ratatui_color(style.foreground);
                Span::styled(
                    text.to_string(),
                    ratatui::style::Style::default().fg(fg),
                )
            })
            .collect();
        result.push(spans);
    }

    result
}

fn syntect_to_ratatui_color(color: syntect::highlighting::Color) -> Color {
    Color::Rgb(color.r, color.g, color.b)
}
