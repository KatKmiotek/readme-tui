use layout::get_layout;
use ratatui::Frame;
mod layout;

pub fn ui(frame: &mut Frame) {
    get_layout(frame);
}
