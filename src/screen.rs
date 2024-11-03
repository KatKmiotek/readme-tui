use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Clear, List, ListItem, ListState},
    Frame,
};

pub struct Screen {
    items: Vec<String>,
    list_state: ListState,
    show_popup: bool,
}
impl Screen {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            items: vec![
                "Item 1".to_string(),
                "Item 2".to_string(),
                "Item 3".to_string(),
            ],
            list_state,
            show_popup: false,
        }
    }
    pub fn get_layout(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
            .split(area);
        let navigation_menu = layout[0];
        let content_area = layout[1];

        let content_block = Block::new()
            .borders(Borders::ALL)
            .title("Content")
            .border_style(Style::default().fg(Color::Red));
        frame.render_widget(content_block, content_area);

        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();
        let list = List::new(items)
            .block(Block::bordered().title("Topics"))
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, navigation_menu, &mut self.list_state);
        if self.show_popup {
            let popup_area = Self::set_popup_area(area, 80, 30);
            let popup_block = Block::default()
                .borders(Borders::ALL)
                .title("Popup")
                .title_alignment(ratatui::layout::Alignment::Center)
                .border_style(Style::default().fg(Color::Blue))
                .style(Style::default().bg(Color::LightGreen).fg(Color::Black))
                .title_style(
                    Style::default()
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                );

            frame.render_widget(Clear, popup_area);
            frame.render_widget(popup_block, popup_area);
        }
    }
    pub fn next(&mut self) {
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
    }
    pub fn previous(&mut self) {
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
    }
    pub fn toggle_popup(&mut self) {
        self.show_popup = !self.show_popup;
    }
    fn set_popup_area(area: Rect, vertical_percentage: u16, horizontal_percentage: u16) -> Rect {
        let vertical =
            Layout::vertical([Constraint::Percentage(horizontal_percentage)]).flex(Flex::Center);
        let horizontal =
            Layout::horizontal([Constraint::Percentage(vertical_percentage)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}
