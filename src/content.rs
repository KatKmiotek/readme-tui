use color_eyre::eyre::Result;
use ratatui::{
    layout::{Position, Rect},
    style::{Color, Style},
    widgets::{
        Block, Borders, Clear, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState,
    },
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
    scroll_offset: usize,
    visible_height: usize,
    pub vertical_scroll_state: ScrollbarState,
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
            scroll_offset: 0,
            visible_height: 0,
            vertical_scroll_state: ScrollbarState::default(),
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
            if let Some(saved_content) = self.file_to_save.get(&selected_topic) {
                self.content_input = saved_content.clone();
            } else if let Some(content) = self.topic_content_map.get(&selected_topic) {
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
        self.visible_height = (area.height as usize).saturating_sub(2);

        if !self.enable_insert_mode {
            if let Some(selected_index) = list_state.selected() {
                self.select_placeholder(selected_index);
            }
        } else if let Some(selected_index) = list_state.selected() {
            self.save_content_for_current_topic(selected_index);
        }

        self.adjust_scroll();

        if self.enable_insert_mode {
            let cursor_y = self.cursor_index_y.saturating_sub(self.scroll_offset);
            if cursor_y < self.visible_height {
                frame.set_cursor_position(Position::new(
                    area.x + self.cursor_index_x as u16 + 1,
                    area.y + cursor_y as u16 + 1,
                ));
            }
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red));

        let inner_area = block.inner(area);
        frame.render_widget(Clear, inner_area);

        let visible_content: Vec<&str> = self
            .content_input
            .iter()
            .skip(self.scroll_offset)
            .take(self.visible_height)
            .map(|s| s.as_str())
            .collect();

        let content_str = visible_content.join("\n");
        let content_paragraph = Paragraph::new(content_str).block(block);
        self.vertical_scroll_state = self
            .vertical_scroll_state
            .content_length(self.content_input.len())
            .position(self.scroll_offset)
            .viewport_content_length(self.visible_height);

        frame.render_widget(content_paragraph, area);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut self.vertical_scroll_state,
        );
    }

    fn adjust_scroll(&mut self) {
        if self.cursor_index_y >= self.scroll_offset + self.visible_height {
            self.scroll_offset = self.cursor_index_y.saturating_sub(self.visible_height - 1);
        } else if self.cursor_index_y < self.scroll_offset {
            self.scroll_offset = self.cursor_index_y;
        }
        let max_scroll = self.content_input.len().saturating_sub(self.visible_height);
        self.scroll_offset = self.scroll_offset.min(max_scroll);
    }

    pub fn toggle_insert(&mut self) {
        self.enable_insert_mode = !self.enable_insert_mode;
        self.cursor_index_y = 0;
        self.cursor_index_x = 0;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_index_y < self.content_input.len() {
            let line = &mut self.content_input[self.cursor_index_y];

            if self.cursor_index_x > 0 {
                line.remove(self.cursor_index_x - 1);
                self.cursor_index_x -= 1;
            } else if self.cursor_index_y > 0 {
                let current_line = self.content_input.remove(self.cursor_index_y);
                self.cursor_index_y -= 1;
                if let Some(previous_line) = self.content_input.get_mut(self.cursor_index_y) {
                    self.cursor_index_x = previous_line.len();
                    previous_line.push_str(&current_line);
                }
            }
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_index_x > 0 {
            self.cursor_index_x -= 1;
        } else if self.cursor_index_y > 0 {
            self.cursor_index_y -= 1;
            if let Some(line) = self.content_input.get(self.cursor_index_y) {
                self.cursor_index_x = line.len();
            }
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_index_y < self.content_input.len() {
            let current_line_length = self.content_input[self.cursor_index_y].len();
            if self.cursor_index_x < current_line_length {
                self.cursor_index_x += 1;
            } else if self.cursor_index_y + 1 < self.content_input.len() {
                self.cursor_index_y += 1;
                self.cursor_index_x = 0;
            }
        }
    }

    pub fn move_cursor_up(&mut self) {
        if self.cursor_index_y > 0 {
            self.cursor_index_y -= 1;
            let line_length = self.content_input[self.cursor_index_y].len();
            self.cursor_index_x = self.cursor_index_x.min(line_length);
            self.adjust_scroll();
        }
    }

    pub fn move_cursor_down(&mut self) {
        if self.cursor_index_y + 1 < self.content_input.len() {
            self.cursor_index_y += 1;
            let line_length = self.content_input[self.cursor_index_y].len();
            self.cursor_index_x = self.cursor_index_x.min(line_length);
            self.adjust_scroll();
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        if self.content_input.len() > self.visible_height {
            self.scroll_offset = self.content_input.len() - self.visible_height;
            self.cursor_index_y = self.content_input.len().saturating_sub(1);
            self.cursor_index_x = self
                .content_input
                .last()
                .map(|line| line.len())
                .unwrap_or(0);
        }

        self.vertical_scroll_state = self
            .vertical_scroll_state
            .position(self.scroll_offset)
            .content_length(self.content_input.len())
            .viewport_content_length(self.visible_height);
    }

    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
        self.cursor_index_y = 0;
        self.cursor_index_x = 0;

        self.vertical_scroll_state = self
            .vertical_scroll_state
            .position(0)
            .content_length(self.content_input.len())
            .viewport_content_length(self.visible_height);
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

    pub fn handle_enter(&mut self) {
        if self.cursor_index_y >= self.content_input.len() {
            self.content_input.push(String::new());
            self.cursor_index_y += 1;
            self.cursor_index_x = 0;
            return;
        }
        let current_line = self.content_input[self.cursor_index_y].clone();
        let (before_cursor, after_cursor) = current_line.split_at(self.cursor_index_x);

        self.content_input[self.cursor_index_y] = before_cursor.to_string();
        self.content_input
            .insert(self.cursor_index_y + 1, after_cursor.to_string());

        self.cursor_index_y += 1;
        self.cursor_index_x = 0;
    }
}
