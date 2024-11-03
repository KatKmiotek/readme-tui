use ratatui::{layout::{Constraint, Direction, Layout}, style::{Style, Stylize}, widgets::{Block, Borders, List, ListDirection}, Frame};

pub fn get_layout(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(frame.area());
    let navigation_menu = layout[0];
    let content_area = layout[1];
    let content_block = Block::new().borders(Borders::ALL).title("Content");

    frame.render_widget(content_block, content_area);
    let items = ["Item 1", "Item 2", "Item 3"];
    let list = List::new(items)
        .block(Block::bordered().title("Topics"))
        .style(Style::new().white())
        .highlight_style(Style::new().italic())
        .highlight_symbol(">>")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);
    frame.render_widget(list, navigation_menu);
}
