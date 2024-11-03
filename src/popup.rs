use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Padding, Paragraph},
    Frame,
};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PopupButton {
    Cancel,
    ExitWithoutSaving,
    ExitWithSave,
}

impl PopupButton {
    pub fn label(&self) -> &'static str {
        match self {
            PopupButton::Cancel => "Cancel",
            PopupButton::ExitWithoutSaving => "Exit without saving",
            PopupButton::ExitWithSave => "Exit with save",
        }
    }
}
pub fn show_popup(frame: &mut Frame, area: Rect) {
    let vertical = Layout::vertical([Constraint::Percentage(30)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(80)]).flex(Flex::Center);
    let [popup_area] = vertical.areas(area);
    let [popup_area] = horizontal.areas(popup_area);
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Blue));

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);

    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(popup_area);
    let popup_text = Paragraph::new(
        "You are about to exit program. What would you like to do with current changes?",
    )
    .alignment(Alignment::Center);
    frame.render_widget(popup_text, inner_chunks[0]);

    let button_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(inner_chunks[1]);

    let button_styles = [
        Style::default().fg(Color::Black).bg(Color::Gray),
        Style::default().fg(Color::Black).bg(Color::Red),
        Style::default().fg(Color::Black).bg(Color::Green),
    ];
    let button_labels = ["Cancel", "Exit without saving", "Exit with save"];
    let button_text = Paragraph::new(button_labels[0]).block(
        Block::new()
            .style(Style::new().bg(Color::Black))
            .padding(Padding::new(
                button_chunks[0].width / 2,
                0,
                button_chunks[0].height / 2,
                0,
            ))
            .style(button_styles[0]),
    );
    frame.render_widget(button_text, button_chunks[0]);
}
