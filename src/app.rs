use crate::fs_ops::get_current_dir;
use crate::viewer::ViewerState;
use ratatui::widgets::TableState;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::time::Instant;

pub struct AppState {
    pub is_error_displayed: bool,
    pub is_f1_displayed: bool,
    pub is_f2_displayed: bool,
    pub is_f7_displayed: bool,
    pub is_left_active: bool,
    pub dir_left: PathBuf,
    pub dir_right: PathBuf,
    pub page_size: u16,
    pub state_left: TableState,
    pub state_right: TableState,
    pub children_left: Vec<Item>,
    pub children_right: Vec<Item>,
    pub error_message: String,
    pub rename_input: String,
    pub rename_character_index: usize,
    pub create_input: String,
    pub create_character_index: usize,
    pub is_f8_displayed: bool,
    pub delete_item_name: String,
    pub delete_item_is_dir: bool,
    pub search_input: String,
    pub cached_clock: String,
    pub cached_separator_height: u16,
    pub cached_separator: String,
    pub is_f3_displayed: bool,
    pub viewer_state: Option<ViewerState>,
    pub viewer_viewport_height: usize,
    pub is_f4_displayed: bool,
    pub editor_state: Option<EditorState>,
    pub editor_viewport_height: usize,
    pub is_f5_displayed: bool,
    pub copy_source_path: PathBuf,
    pub copy_dest_path: PathBuf,
    pub copy_is_dir: bool,
    pub is_f6_displayed: bool,
    pub move_source_path: PathBuf,
    pub move_dest_path: PathBuf,
    pub move_is_dir: bool,
    pub selected_left: HashSet<usize>,
    pub selected_right: HashSet<usize>,
    pub dir_sizes: HashMap<PathBuf, u64>,
    pub last_click_time: Option<Instant>,
    pub last_click_pos: (u16, u16),
    pub is_editor_save_prompt: bool,
}

use ratatui::text::Span;

#[derive(Clone)]
pub struct EditorState {
    pub file_path: PathBuf,
    pub lines: Vec<String>,
    pub highlighted_lines: Vec<Vec<Span<'static>>>,
    pub cursor_line: usize,
    pub cursor_col: usize,
    pub scroll_offset: usize,
    pub modified: bool,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name_full: String,
    pub name: String,
    pub extension: String,
    pub is_dir: bool,
    pub size: String,
    pub modified: String,
}

impl AppState {
    pub fn new() -> Self {
        let mut state_left = TableState::default();
        state_left.select(Some(1));
        let mut state_right = TableState::default();
        state_right.select(Some(1));
        let mut dir_root = PathBuf::new();

        let mut is_error_displayed = false;
        let mut error_message = String::new();

        match get_current_dir() {
            Ok(root) => dir_root = root,
            Err(e) => {
                is_error_displayed = true;
                error_message = e.to_string();
            }
        }

        Self {
            is_error_displayed,
            is_f1_displayed: false,
            is_f2_displayed: false,
            is_f7_displayed: false,
            is_left_active: true,
            dir_left: dir_root.clone(),
			dir_right: dir_root.clone(),
            page_size: 0,
            state_left,
            state_right,
            children_left: Vec::<Item>::new(),
            children_right: Vec::<Item>::new(),
            error_message,
            rename_input: String::new(),
            rename_character_index: 0,
            create_input: String::new(),
            create_character_index: 0,
            is_f8_displayed: false,
            delete_item_name: String::new(),
            delete_item_is_dir: false,
            search_input: String::new(),
            cached_clock: String::new(),
            cached_separator_height: 0,
            cached_separator: String::new(),
            is_f3_displayed: false,
            viewer_state: None,
            viewer_viewport_height: 0,
            is_f4_displayed: false,
            editor_state: None,
            editor_viewport_height: 0,
            is_f5_displayed: false,
            copy_source_path: PathBuf::new(),
            copy_dest_path: PathBuf::new(),
            copy_is_dir: false,
            is_f6_displayed: false,
            move_source_path: PathBuf::new(),
            move_dest_path: PathBuf::new(),
            move_is_dir: false,
            selected_left: HashSet::new(),
            selected_right: HashSet::new(),
            dir_sizes: HashMap::new(),
            last_click_time: None,
            last_click_pos: (0, 0),
            is_editor_save_prompt: false,
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.rename_character_index.saturating_sub(1);
        self.rename_character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.rename_character_index.saturating_add(1);
        self.rename_character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn move_cursor_end(&mut self) {
        let cursor_moved_right = self.rename_character_index.saturating_add(self.rename_input.len());
        self.rename_character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.rename_input.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.rename_input.chars().count())
    }

    pub fn byte_index(&self) -> usize {
        self.rename_input.char_indices().map(|(i, _)| i).nth(self.rename_character_index).unwrap_or(self.rename_input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.rename_character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.rename_character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.rename_input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.rename_input.chars().skip(current_index);
            self.rename_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn reset_cursor(&mut self) {
        self.rename_character_index = 0;
    }

    pub fn reset_rename(&mut self) {
        self.rename_character_index = 0;
        self.rename_input = String::new();
        self.is_f2_displayed = false;
    }

    pub fn display_error(&mut self, message: String) {
        self.is_error_displayed = true;
        self.error_message = message;
    }

    pub fn reset_error(&mut self) {
        self.is_error_displayed = false;
        self.error_message = String::new();
    }

    pub fn reset_create(&mut self) {
        self.is_f7_displayed = false;
        self.create_input.clear();
        self.create_character_index = 0;
    }

    pub fn create_move_cursor_left(&mut self) {
        let cursor_moved_left = self.create_character_index.saturating_sub(1);
        self.create_character_index = self.create_clamp_cursor(cursor_moved_left);
    }

    pub fn create_move_cursor_right(&mut self) {
        let cursor_moved_right = self.create_character_index.saturating_add(1);
        self.create_character_index = self.create_clamp_cursor(cursor_moved_right);
    }

    pub fn create_enter_char(&mut self, new_char: char) {
        let index = self.create_byte_index();
        self.create_input.insert(index, new_char);
        self.create_move_cursor_right();
    }

    pub fn create_clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.create_input.chars().count())
    }

    pub fn create_byte_index(&self) -> usize {
        self.create_input.char_indices().map(|(i, _)| i).nth(self.create_character_index).unwrap_or(self.create_input.len())
    }

    pub fn create_delete_char(&mut self) {
        let is_not_cursor_leftmost = self.create_character_index != 0;
        if is_not_cursor_leftmost {
            let current_index = self.create_character_index;
            let from_left_to_current_index = current_index - 1;
            let before_char_to_delete = self.create_input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.create_input.chars().skip(current_index);
            self.create_input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.create_move_cursor_left();
        }
    }

    pub fn get_create_input(&self) -> String {
        let (p1, p2) = self.create_input.split_at(self.create_character_index);
        format!("{}_{}",  p1, p2)
    }

    pub fn get_rename_input(&mut self) -> String {
        let (p1, p2) = self.rename_input.split_at(self.rename_character_index);
        return format!("{}{}{}", p1, "_", p2);
    }

    // Quick search methods
    pub fn search_add_char(&mut self, c: char) {
        self.search_input.push(c);
        self.jump_to_first_match();
    }

    pub fn search_backspace(&mut self) {
        if !self.search_input.is_empty() {
            self.search_input.pop();
            if self.search_input.is_empty() {
                // Search cleared, do nothing
            } else {
                self.jump_to_first_match();
            }
        }
    }

    pub fn search_clear(&mut self) {
        self.search_input.clear();
    }

    pub fn jump_to_first_match(&mut self) {
        if self.search_input.is_empty() {
            return;
        }

        let search_lower = self.search_input.to_lowercase();
        let children = if self.is_left_active { &self.children_left } else { &self.children_right };
        let state = if self.is_left_active { &mut self.state_left } else { &mut self.state_right };

        if let Some(index) = children.iter().position(|item| item.name_full.to_lowercase().starts_with(&search_lower)) {
            state.select(Some(index));
        }
    }

    pub fn jump_to_next_match(&mut self) {
        if self.search_input.is_empty() {
            return;
        }

        let search_lower = self.search_input.to_lowercase();
        let children = if self.is_left_active { &self.children_left } else { &self.children_right };
        let state = if self.is_left_active { &mut self.state_left } else { &mut self.state_right };
        let current_index = state.selected().unwrap_or(0);

        // Find next match after current position
        for i in (current_index + 1)..children.len() {
            if children[i].name_full.to_lowercase().starts_with(&search_lower) {
                state.select(Some(i));
                return;
            }
        }

        // Wrap around to beginning
        for i in 0..=current_index {
            if children[i].name_full.to_lowercase().starts_with(&search_lower) {
                state.select(Some(i));
                return;
            }
        }
    }

    pub fn jump_to_prev_match(&mut self) {
        if self.search_input.is_empty() {
            return;
        }

        let search_lower = self.search_input.to_lowercase();
        let children = if self.is_left_active { &self.children_left } else { &self.children_right };
        let state = if self.is_left_active { &mut self.state_left } else { &mut self.state_right };
        let current_index = state.selected().unwrap_or(0);

        // Find previous match before current position
        for i in (0..current_index).rev() {
            if children[i].name_full.to_lowercase().starts_with(&search_lower) {
                state.select(Some(i));
                return;
            }
        }

        // Wrap around to end
        for i in (current_index..children.len()).rev() {
            if children[i].name_full.to_lowercase().starts_with(&search_lower) {
                state.select(Some(i));
                return;
            }
        }
    }

    pub fn reset_delete(&mut self) {
        self.is_f8_displayed = false;
        self.delete_item_name.clear();
        self.delete_item_is_dir = false;
    }

    pub fn open_viewer(&mut self, file_path: PathBuf) -> Result<(), String> {
        use crate::viewer::load_file_content;

        match load_file_content(&file_path) {
            Ok(state) => {
                self.viewer_state = Some(state);
                self.is_f3_displayed = true;
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn close_viewer(&mut self) {
        self.is_f3_displayed = false;
        self.viewer_state = None;
    }

    pub fn viewer_scroll_down(&mut self) {
        if let Some(state) = &mut self.viewer_state {
            let max_offset = state.total_lines.saturating_sub(self.viewer_viewport_height);
            state.scroll_offset = (state.scroll_offset + 1).min(max_offset);
        }
    }

    pub fn viewer_scroll_up(&mut self) {
        if let Some(state) = &mut self.viewer_state {
            state.scroll_offset = state.scroll_offset.saturating_sub(1);
        }
    }

    pub fn viewer_page_down(&mut self) {
        if let Some(state) = &mut self.viewer_state {
            let max_offset = state.total_lines.saturating_sub(self.viewer_viewport_height);
            state.scroll_offset = (state.scroll_offset + self.viewer_viewport_height).min(max_offset);
        }
    }

    pub fn viewer_page_up(&mut self) {
        if let Some(state) = &mut self.viewer_state {
            state.scroll_offset = state.scroll_offset.saturating_sub(self.viewer_viewport_height);
        }
    }

    pub fn viewer_home(&mut self) {
        if let Some(state) = &mut self.viewer_state {
            state.scroll_offset = 0;
        }
    }

    pub fn viewer_end(&mut self) {
        if let Some(state) = &mut self.viewer_state {
            let max_offset = state.total_lines.saturating_sub(self.viewer_viewport_height);
            state.scroll_offset = max_offset;
        }
    }

    pub fn open_editor(&mut self, file_path: PathBuf) -> Result<(), String> {
        use crate::viewer::{detect_syntax, highlight_content};

        let content = std::fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        let lines = if lines.is_empty() { vec![String::new()] } else { lines };
        let syntax_name = detect_syntax(&file_path);
        let highlighted_lines = highlight_content(&lines, &syntax_name);

        self.editor_state = Some(EditorState {
            file_path,
            lines,
            highlighted_lines,
            cursor_line: 0,
            cursor_col: 0,
            scroll_offset: 0,
            modified: false,
        });
        self.is_f4_displayed = true;
        Ok(())
    }

    pub fn close_editor(&mut self) {
        self.is_f4_displayed = false;
        self.editor_state = None;
    }

    pub fn editor_rehighlight(&mut self) {
        if let Some(state) = &mut self.editor_state {
            let syntax_name = crate::viewer::detect_syntax(&state.file_path);
            state.highlighted_lines = crate::viewer::highlight_content(&state.lines, &syntax_name);
        }
    }

    pub fn editor_cursor_up(&mut self) {
        if let Some(state) = &mut self.editor_state {
            if state.cursor_line > 0 {
                state.cursor_line -= 1;
                let line_len = state.lines[state.cursor_line].chars().count();
                state.cursor_col = state.cursor_col.min(line_len);
                // Adjust scroll if cursor goes above viewport
                if state.cursor_line < state.scroll_offset {
                    state.scroll_offset = state.cursor_line;
                }
            }
        }
    }

    pub fn editor_cursor_down(&mut self) {
        if let Some(state) = &mut self.editor_state {
            if state.cursor_line < state.lines.len().saturating_sub(1) {
                state.cursor_line += 1;
                let line_len = state.lines[state.cursor_line].chars().count();
                state.cursor_col = state.cursor_col.min(line_len);
                // Adjust scroll if cursor goes below viewport
                if state.cursor_line >= state.scroll_offset + self.editor_viewport_height {
                    state.scroll_offset = state.cursor_line.saturating_sub(self.editor_viewport_height) + 1;
                }
            }
        }
    }

    pub fn editor_cursor_left(&mut self) {
        if let Some(state) = &mut self.editor_state {
            if state.cursor_col > 0 {
                state.cursor_col -= 1;
            } else if state.cursor_line > 0 {
                state.cursor_line -= 1;
                state.cursor_col = state.lines[state.cursor_line].chars().count();
                if state.cursor_line < state.scroll_offset {
                    state.scroll_offset = state.cursor_line;
                }
            }
        }
    }

    pub fn editor_cursor_right(&mut self) {
        if let Some(state) = &mut self.editor_state {
            let line_len = state.lines[state.cursor_line].chars().count();
            if state.cursor_col < line_len {
                state.cursor_col += 1;
            } else if state.cursor_line < state.lines.len().saturating_sub(1) {
                state.cursor_line += 1;
                state.cursor_col = 0;
                if state.cursor_line >= state.scroll_offset + self.editor_viewport_height {
                    state.scroll_offset = state.cursor_line.saturating_sub(self.editor_viewport_height) + 1;
                }
            }
        }
    }

    pub fn editor_home(&mut self) {
        if let Some(state) = &mut self.editor_state {
            state.cursor_col = 0;
        }
    }

    pub fn editor_end(&mut self) {
        if let Some(state) = &mut self.editor_state {
            state.cursor_col = state.lines[state.cursor_line].chars().count();
        }
    }

    pub fn editor_page_up(&mut self) {
        if let Some(state) = &mut self.editor_state {
            let page = self.editor_viewport_height.saturating_sub(1);
            state.cursor_line = state.cursor_line.saturating_sub(page);
            state.scroll_offset = state.scroll_offset.saturating_sub(page);
            let line_len = state.lines[state.cursor_line].chars().count();
            state.cursor_col = state.cursor_col.min(line_len);
        }
    }

    pub fn editor_page_down(&mut self) {
        if let Some(state) = &mut self.editor_state {
            let page = self.editor_viewport_height.saturating_sub(1);
            let max_line = state.lines.len().saturating_sub(1);
            state.cursor_line = (state.cursor_line + page).min(max_line);
            let max_scroll = state.lines.len().saturating_sub(self.editor_viewport_height);
            state.scroll_offset = (state.scroll_offset + page).min(max_scroll);
            let line_len = state.lines[state.cursor_line].chars().count();
            state.cursor_col = state.cursor_col.min(line_len);
        }
    }

    pub fn editor_insert_char(&mut self, c: char) {
        if let Some(state) = &mut self.editor_state {
            let line = &mut state.lines[state.cursor_line];
            let byte_idx = line.char_indices().nth(state.cursor_col).map(|(i, _)| i).unwrap_or(line.len());
            line.insert(byte_idx, c);
            state.cursor_col += 1;
            state.modified = true;
        }
        self.editor_rehighlight();
    }

    pub fn editor_backspace(&mut self) {
        let mut changed = false;
        if let Some(state) = &mut self.editor_state {
            if state.cursor_col > 0 {
                let line = &mut state.lines[state.cursor_line];
                let byte_idx = line.char_indices().nth(state.cursor_col - 1).map(|(i, _)| i).unwrap_or(0);
                let end_idx = line.char_indices().nth(state.cursor_col).map(|(i, _)| i).unwrap_or(line.len());
                line.replace_range(byte_idx..end_idx, "");
                state.cursor_col -= 1;
                state.modified = true;
                changed = true;
            } else if state.cursor_line > 0 {
                // Join with previous line
                let current_line = state.lines.remove(state.cursor_line);
                state.cursor_line -= 1;
                state.cursor_col = state.lines[state.cursor_line].chars().count();
                state.lines[state.cursor_line].push_str(&current_line);
                state.modified = true;
                changed = true;
                if state.cursor_line < state.scroll_offset {
                    state.scroll_offset = state.cursor_line;
                }
            }
        }
        if changed {
            self.editor_rehighlight();
        }
    }

    pub fn editor_delete(&mut self) {
        let mut changed = false;
        if let Some(state) = &mut self.editor_state {
            let line_len = state.lines[state.cursor_line].chars().count();
            if state.cursor_col < line_len {
                let line = &mut state.lines[state.cursor_line];
                let byte_idx = line.char_indices().nth(state.cursor_col).map(|(i, _)| i).unwrap_or(line.len());
                let end_idx = line.char_indices().nth(state.cursor_col + 1).map(|(i, _)| i).unwrap_or(line.len());
                line.replace_range(byte_idx..end_idx, "");
                state.modified = true;
                changed = true;
            } else if state.cursor_line < state.lines.len().saturating_sub(1) {
                // Join with next line
                let next_line = state.lines.remove(state.cursor_line + 1);
                state.lines[state.cursor_line].push_str(&next_line);
                state.modified = true;
                changed = true;
            }
        }
        if changed {
            self.editor_rehighlight();
        }
    }

    pub fn editor_enter(&mut self) {
        if let Some(state) = &mut self.editor_state {
            let line = &mut state.lines[state.cursor_line];
            let byte_idx = line.char_indices().nth(state.cursor_col).map(|(i, _)| i).unwrap_or(line.len());
            let new_line = line[byte_idx..].to_string();
            line.truncate(byte_idx);
            state.cursor_line += 1;
            state.lines.insert(state.cursor_line, new_line);
            state.cursor_col = 0;
            state.modified = true;
            if state.cursor_line >= state.scroll_offset + self.editor_viewport_height {
                state.scroll_offset = state.cursor_line.saturating_sub(self.editor_viewport_height) + 1;
            }
        }
        self.editor_rehighlight();
    }

    pub fn editor_save(&mut self) -> Result<(), String> {
        if let Some(state) = &mut self.editor_state {
            let content = state.lines.join("\n");
            std::fs::write(&state.file_path, content).map_err(|e| e.to_string())?;
            state.modified = false;
        }
        Ok(())
    }

    pub fn editor_is_modified(&self) -> bool {
        self.editor_state.as_ref().map(|s| s.modified).unwrap_or(false)
    }

    pub fn reset_copy(&mut self) {
        self.is_f5_displayed = false;
        self.copy_source_path = PathBuf::new();
        self.copy_dest_path = PathBuf::new();
        self.copy_is_dir = false;
    }

    pub fn reset_move(&mut self) {
        self.is_f6_displayed = false;
        self.move_source_path = PathBuf::new();
        self.move_dest_path = PathBuf::new();
        self.move_is_dir = false;
    }

    pub fn toggle_selection(&mut self) {
        self.toggle_selection_inner(true);
    }

    pub fn toggle_selection_no_size(&mut self) {
        self.toggle_selection_inner(false);
    }

    fn toggle_selection_inner(&mut self, calculate_size: bool) {
        use crate::fs_ops::calculate_dir_size;

        let mut error_msg: Option<String> = None;
        let mut dir_size_result: Option<(PathBuf, u64)> = None;

        {
            let (state, children, selected_set, current_dir) = if self.is_left_active {
                (&mut self.state_left, &self.children_left, &mut self.selected_left, &self.dir_left)
            } else {
                (&mut self.state_right, &self.children_right, &mut self.selected_right, &self.dir_right)
            };

            if let Some(index) = state.selected() {
                // Don't allow selecting ".." entry
                if index < children.len() && children[index].name != ".." {
                    let item = &children[index];

                    if selected_set.contains(&index) {
                        selected_set.remove(&index);
                    } else {
                        selected_set.insert(index);

                        // Calculate directory size when selecting
                        if calculate_size && item.is_dir {
                            let mut full_path = current_dir.clone();
                            full_path.push(&item.name_full);
                            match calculate_dir_size(&full_path) {
                                Ok(size) => {
                                    dir_size_result = Some((full_path, size));
                                }
                                Err(e) => {
                                    error_msg = Some(format!("Cannot calculate size: {}", e));
                                }
                            }
                        }
                    }
                }

                // Move to next item
                let len = children.len();
                if len > 0 {
                    let next = if index >= len - 1 { index } else { index + 1 };
                    state.select(Some(next));
                }
            }
        }

        // Handle results after borrows are released
        if let Some((path, size)) = dir_size_result {
            self.dir_sizes.insert(path, size);
        }
        if let Some(msg) = error_msg {
            self.display_error(msg);
        }
    }

    pub fn clear_all_selections(&mut self) {
        self.selected_left.clear();
        self.selected_right.clear();
    }

    pub fn clear_active_selections(&mut self) {
        if self.is_left_active {
            self.selected_left.clear();
        } else {
            self.selected_right.clear();
        }
    }
}
