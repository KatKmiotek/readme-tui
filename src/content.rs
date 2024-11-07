use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, ListState, Paragraph},
    Frame,
};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum TopicListItem {
    Tutorials,
    Guides,
    Explanation,
    Reference,
}

pub struct Content {
    pub content_input: String,
    topic_content_map: HashMap<TopicListItem, String>,
    pub enable_insert_mode: bool,
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
            TopicListItem::Tutorials,
            "Placeholder content for Tutorials".to_string(),
        );
        topic_content_map.insert(
            TopicListItem::Guides,
            "Placeholder content for Guides".to_string(),
        );
        topic_content_map.insert(
            TopicListItem::Explanation,
            "Placeholder content for Explanation".to_string(),
        );
        topic_content_map.insert(
            TopicListItem::Reference,
            "Placeholder content for Reference".to_string(),
        );

        Self {
            content_input: String::new(),
            topic_content_map,
            enable_insert_mode: false,
        }
    }
    // refactor should be handled by render
    pub fn select_topic(&mut self, index: usize) {
        let selected_topic = match index {
            0 => TopicListItem::Tutorials,
            1 => TopicListItem::Guides,
            2 => TopicListItem::Explanation,
            3 => TopicListItem::Reference,
            _ => return,
        };

        if let Some(content) = self.topic_content_map.get(&selected_topic) {
            self.content_input = content.clone();
        }
    }
    pub fn render(&mut self, frame: &mut Frame, area: Rect, list_state: &ListState) {
        if self.enable_insert_mode && self.content_input.is_empty() {
            // because of 2nd condition after deleting last letter input resets - need something better
            if let Some(selected_index) = list_state.selected() {
                let selected_topic = match selected_index {
                    0 => TopicListItem::Tutorials,
                    1 => TopicListItem::Guides,
                    2 => TopicListItem::Explanation,
                    3 => TopicListItem::Reference,
                    _ => return,
                };

                if let Some(content) = self.topic_content_map.get(&selected_topic) {
                    self.content_input = content.clone();
                }
            }
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
        self.enable_insert_mode = !self.enable_insert_mode
    }
}
