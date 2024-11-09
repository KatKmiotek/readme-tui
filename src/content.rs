use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, ListState, Paragraph},
    Frame,
};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum ContentListItem {
    Tutorials,
    Guides,
    Explanation,
    Reference,
}

pub struct Content {
    pub content_input: String,
    topic_content_map: HashMap<ContentListItem, String>,
    pub enable_insert_mode: bool,
    pub file_to_save: HashMap<ContentListItem, String>,
}

impl Default for Content {
    fn default() -> Self {
        Self::new()
    }
}

impl Content {
    pub fn new() -> Self {
        let mut topic_content_map = HashMap::new();
        topic_content_map.insert(
            ContentListItem::Tutorials,
            "Placeholder content for Tutorials".to_string(),
        );
        topic_content_map.insert(
            ContentListItem::Guides,
            "Placeholder content for Guides".to_string(),
        );
        topic_content_map.insert(
            ContentListItem::Explanation,
            "Placeholder content for Explanation".to_string(),
        );
        topic_content_map.insert(
            ContentListItem::Reference,
            "Placeholder content for Reference".to_string(),
        );

        Self {
            content_input: String::new(),
            topic_content_map,
            enable_insert_mode: false,
            file_to_save: HashMap::new(),
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
                    self.content_input = content.clone();
                }
            }
        }
    }

    pub fn save_content_for_current_topic(&mut self, index: usize) {
        if let Some(selected_topic) = Content::get_content_for_index(index) {
            self.file_to_save
                .insert(selected_topic.clone(), self.content_input.clone());
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

        let content_paragraph = Paragraph::new(self.content_input.as_str())
            .block(Block::default().borders(Borders::ALL).title(title));

        frame.render_widget(content_paragraph, area);
    }
    pub fn toggle_insert(&mut self) {
        self.enable_insert_mode = !self.enable_insert_mode;
    }
}
