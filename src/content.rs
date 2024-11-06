use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Content {
    pub content_input: String,
}
impl Default for Content {
    fn default() -> Self {
        Self::new()
    }
}

impl Content {
    pub fn new() -> Self {
        Self {
            content_input: String::new(),
        }
    }
    pub fn enable_editing(&mut self, frame: &mut Frame, area: Rect) {
        let place_holder = Paragraph::new("Editing....");
        frame.render_widget(place_holder, area);
        let content = Paragraph::new(self.content_input.as_str())
            .block(Block::default().borders(Borders::ALL).title("Content"));
        frame.render_widget(content, area);
    }
}
