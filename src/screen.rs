use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::content::Content;
use crate::popup::Popup;

pub struct Screen {
    items: Vec<String>,
    list_state: ListState,
    pub show_popup: bool,
    pub enable_insert_mode: bool,
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

impl Screen {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            items: vec![
                "Project Name".to_string(),
                "Tutorials".to_string(),
                "How-to Guides".to_string(),
                "Explanation".to_string(),
                "Reference".to_string(),
            ],
            list_state,
            show_popup: false,
            enable_insert_mode: false,
        }
    }
    pub fn get_layout(&mut self, frame: &mut Frame, popup: &mut Popup, content: &mut Content) {
        let area = frame.area();
        let all = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(5), Constraint::Percentage(95)])
            .split(area);
        let top_area = all[0];
        let navbar = Block::new().title("CLI DOCS ".bold());
        let additional_instruction = if content.enable_insert_mode {
            "                      Use ◄ ▲ ▼ ► to navigate, F1 - page up, F2 - page down, press ESC to exit"
        } else {
            "                      Use ▲ ▼  to navigate, press i to edit, press ESC to exit"
        };
        frame.render_widget(navbar.title(additional_instruction), top_area);
        let main_area = all[1];
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(main_area);
        let navigation_menu = layout[0];
        let content_area = layout[1];

        let content_text = content.content_input.join("\n");
        let content_block = Paragraph::new(content_text.as_str()).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        );
        frame.render_widget(content_block, content_area);

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();
        let list = List::new(items)
            .block(Block::bordered())
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, navigation_menu, &mut self.list_state);
        if self.show_popup {
            popup.show_popup(frame, area);
        } else {
            content.render(frame, content_area, &self.list_state);
            if content.enable_insert_mode {}
        }
    }

    pub fn next(&mut self, content: &mut Content) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        content.select_placeholder(i)
    }
    pub fn previous(&mut self, content: &mut Content) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
        content.select_placeholder(i)
    }
    pub fn toggle_popup(&mut self) {
        self.show_popup = !self.show_popup;
    }
}

#[cfg(test)]
mod tests {
    use crate::content::Content;

    use super::Screen;

    #[test]
    fn test_default_selection() {
        let screen = Screen::new();
        assert_eq!(screen.list_state.selected(), Some(0));
    }

    #[test]
    fn test_next_navigation() {
        let mut screen = Screen::new();
        let mut content = Content::new();
        assert_eq!(screen.list_state.selected(), Some(0));

        screen.next(&mut content);
        assert_eq!(screen.list_state.selected(), Some(1));

        screen.next(&mut content);
        assert_eq!(screen.list_state.selected(), Some(2));

        screen.next(&mut content);
        assert_eq!(screen.list_state.selected(), Some(3));

        screen.next(&mut content);
        assert_eq!(screen.list_state.selected(), Some(4));

        screen.next(&mut content);
        assert_eq!(screen.list_state.selected(), Some(0));
    }

    #[test]
    fn test_previous_navigation() {
        let mut screen = Screen::new();
        let mut content = Content::new();

        assert_eq!(screen.list_state.selected(), Some(0));

        screen.previous(&mut content);
        assert_eq!(screen.list_state.selected(), Some(screen.items.len() - 1));

        screen.previous(&mut content);
        assert_eq!(screen.list_state.selected(), Some(screen.items.len() - 2));
    }

    #[test]
    fn test_toggle_popup() {
        let mut screen = Screen::new();
        assert!(!screen.show_popup);

        screen.toggle_popup();
        assert!(screen.show_popup);

        screen.toggle_popup();
        assert!(!screen.show_popup);
    }

    #[test]
    fn test_screen_items_length() {
        let screen = Screen::new();

        assert_eq!(screen.items.len(), 5);
        assert_eq!(screen.items[0], "Project Name");
        assert_eq!(screen.items[1], "Tutorials");
        assert_eq!(screen.items[2], "How-to Guides");
        assert_eq!(screen.items[3], "Explanation");
        assert_eq!(screen.items[4], "Reference");
    }

    #[test]
    fn test_selection_persistence_after_toggle_popup() {
        let mut screen = Screen::new();
        let mut content = Content::new();
        screen.next(&mut content);
        assert_eq!(screen.list_state.selected(), Some(1));

        screen.toggle_popup();
        assert_eq!(screen.list_state.selected(), Some(1));
    }
}
