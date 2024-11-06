use ratatui::layout::Rect;

pub struct Content {
    editing: bool,
}
impl Default for Content {
    fn default() -> Self {
        Self::new()
    }
}
impl Content {
    pub fn new() -> Self {
        Self { editing: false }
    }
    pub fn enable_editing(&mut self, area: Rect) {}
}
