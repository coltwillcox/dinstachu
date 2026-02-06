use crate::fs_ops::get_current_dir;
use ratatui::widgets::TableState;
use std::path::PathBuf;

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
    pub search_input: String,
    pub cached_clock: String,
    pub cached_separator_height: u16,
    pub cached_separator: String,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name_full: String,
    pub name: String,
    pub extension: String,
    pub is_dir: bool,
    pub size: String,
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
            search_input: String::new(),
            cached_clock: String::new(),
            cached_separator_height: 0,
            cached_separator: String::new(),
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
}
