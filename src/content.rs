use color_eyre::eyre::Result;
use ratatui::{
    layout::{Position, Rect},
    widgets::{Block, Borders, ListState, Paragraph},
    Frame,
};
use std::{collections::HashMap, fs, path::Path};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum ContentListItem {
    Tutorials,
    Guides,
    Explanation,
    Reference,
}

pub struct Content {
    pub content_input: Vec<String>,
    topic_content_map: HashMap<ContentListItem, String>,
    pub enable_insert_mode: bool,
    pub file_to_save: HashMap<ContentListItem, Vec<String>>,
    pub cursor_index_x: usize,
    pub cursor_index_y: usize,
}

impl Default for Content {
    fn default() -> Self {
        Self::new()
    }
}

impl Content {
    pub fn new() -> Self {
        let mut topic_content_map = HashMap::new();
        topic_content_map.insert(ContentListItem::Tutorials, "tutorials.md".to_string());
        topic_content_map.insert(ContentListItem::Guides, "guides.md".to_string());
        topic_content_map.insert(ContentListItem::Explanation, "explanation.md".to_string());
        topic_content_map.insert(ContentListItem::Reference, "reference.md".to_string());

        Self {
            content_input: Vec::new(),
            topic_content_map,
            enable_insert_mode: false,
            file_to_save: HashMap::new(),
            cursor_index_x: 0,
            cursor_index_y: 0,
        }
    }

    fn get_content_for_index(index: usize) -> Option<ContentListItem> {
        match index {
            0 => Some(ContentListItem::Tutorials),
            1 => Some(ContentListItem::Guides),
            2 => Some(ContentListItem::Explanation),
            3 => Some(ContentListItem::Reference),
            _ => None,
        }
    }

    pub fn select_placeholder(&mut self, index: usize) {
        if let Some(selected_topic) = Content::get_content_for_index(index) {
            if let Some(content) = self.topic_content_map.get(&selected_topic) {
                if !self.enable_insert_mode {
                    self.content_input = Content::read_placeholder_from_file(content)
                        .unwrap_or_else(|_| vec!["empty".to_string()]);
                }
            }
        }
    }

    pub fn read_placeholder_from_file(file: &str) -> Result<Vec<String>> {
        let dir_path = Path::new("templates");
        let file_path = dir_path.join(file);
        let data = fs::read_to_string(file_path).expect("Unable to read file");
        let lines: Vec<String> = data.lines().map(|line| line.to_string()).collect();
        Ok(lines)
    }

    pub fn save_content_for_current_topic(&mut self, index: usize) {
        if let Some(selected_topic) = Content::get_content_for_index(index) {
            self.file_to_save
                .insert(selected_topic.clone(), self.content_input.clone());
            if let Some(content) = self.topic_content_map.get_mut(&selected_topic) {
                *content = content.clone();
            }
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, list_state: &ListState) {
        if !self.enable_insert_mode {
            if let Some(selected_index) = list_state.selected() {
                self.select_placeholder(selected_index);
            }
        } else if let Some(selected_index) = list_state.selected() {
            self.save_content_for_current_topic(selected_index);
        }

        let title = if self.enable_insert_mode {
            "Press ESC to exit editor mode."
        } else {
            "Press I to enter editing mode"
        };

        if self.enable_insert_mode && self.cursor_index_y < self.content_input.len() {
            frame.set_cursor_position(Position::new(
                area.x + self.cursor_index_x as u16 + 1,
                area.y + self.cursor_index_y as u16 + 1,
            ));
        }

        let content_str = self.content_input.join("\n");
        let content_paragraph = Paragraph::new(content_str.as_str())
            .block(Block::default().borders(Borders::ALL).title(title));

        frame.render_widget(content_paragraph, area);
    }

    pub fn toggle_insert(&mut self) {
        self.enable_insert_mode = !self.enable_insert_mode;
        self.cursor_index_y = 0;
        self.cursor_index_x = 0;
    }

    pub fn insert_char(&mut self, ch: char) {
        if self.cursor_index_y >= self.content_input.len() {
            self.content_input.push(String::new());
        }
        if let Some(line) = self.content_input.get_mut(self.cursor_index_y) {
            line.insert(self.cursor_index_x, ch);
            self.cursor_index_x += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if let Some(line) = self.content_input.get_mut(self.cursor_index_y) {
            if self.cursor_index_x > 0 {
                line.remove(self.cursor_index_x - 1);
                self.cursor_index_x -= 1;
            }
        }
    }
}
