use ratatui::{
    layout::{Alignment, Constraint, Direction, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
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
            PopupButton::ExitWithoutSaving => "Exit",
            PopupButton::ExitWithSave => "Save",
        }
    }
    pub fn style(&self) -> Style {
        match self {
            PopupButton::Cancel => Style::default().fg(Color::Black).bg(Color::Gray),
            PopupButton::ExitWithoutSaving => Style::default().fg(Color::Black).bg(Color::Red),
            PopupButton::ExitWithSave => Style::default().fg(Color::Black).bg(Color::Green),
        }
    }
}

pub struct Popup {
    selected_button_index: usize,
    buttons: Vec<PopupButton>,
}

impl Default for Popup {
    fn default() -> Self {
        Self::new()
    }
}

impl Popup {
    pub fn new() -> Self {
        Self {
            selected_button_index: 0,
            buttons: vec![
                PopupButton::Cancel,
                PopupButton::ExitWithoutSaving,
                PopupButton::ExitWithSave,
            ],
        }
    }

    pub fn next_button(&mut self) {
        self.selected_button_index = (self.selected_button_index + 1) % 3;
    }

    pub fn previous_button(&mut self) {
        if self.selected_button_index == 0 {
            self.selected_button_index = 2;
        } else {
            self.selected_button_index -= 1;
        }
    }
    pub fn select_button(&self) -> PopupButton {
        match self.selected_button_index {
            0 => PopupButton::Cancel,
            1 => PopupButton::ExitWithoutSaving,
            2 => PopupButton::ExitWithSave,
            _ => unreachable!(),
        }
    }
    pub fn show_popup(&mut self, frame: &mut Frame, area: Rect) {
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
            "You are about to exit the program. What would you like to do with current changes?",
        )
        .block(
            Block::default()
                .borders(Borders::NONE)
                .padding(Padding::new(2, 2, 2, 1)),
        )
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));

        frame.render_widget(popup_text, inner_chunks[0]);
        let button_spaces = Layout::default()
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

        for (i, &button) in self.buttons.iter().enumerate() {
            let button_style = if i == self.selected_button_index {
                button.style().add_modifier(Modifier::BOLD)
            } else {
                button.style()
            };

            let button_text = Paragraph::new(button.label()).style(button_style).block(
                Block::default()
                    .borders(Borders::ALL)
                    .padding(Padding::new(12, 0, 1, 0))
                    .title_alignment(Alignment::Center)
            );

            frame.render_widget(button_text, button_spaces[i]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_selected_button() {
        let popup = Popup::new();
        assert_eq!(popup.select_button(), PopupButton::Cancel);
    }

    #[test]
    fn test_next_button() {
        let mut popup = Popup::new();
        popup.next_button();
        assert_eq!(popup.select_button(), PopupButton::ExitWithoutSaving);

        popup.next_button();
        assert_eq!(popup.select_button(), PopupButton::ExitWithSave);

        popup.next_button();
        assert_eq!(popup.select_button(), PopupButton::Cancel);
    }

    #[test]
    fn test_previous_button() {
        let mut popup = Popup::new();

        popup.previous_button();
        assert_eq!(popup.select_button(), PopupButton::ExitWithSave);

        popup.previous_button();
        assert_eq!(popup.select_button(), PopupButton::ExitWithoutSaving);

        popup.previous_button();
        assert_eq!(popup.select_button(), PopupButton::Cancel);
    }

    #[test]
    fn test_popup_button_labels() {
        let cancel = PopupButton::Cancel;
        let exit_without_saving = PopupButton::ExitWithoutSaving;
        let exit_with_save = PopupButton::ExitWithSave;

        assert_eq!(cancel.label(), "Cancel");
        assert_eq!(exit_without_saving.label(), "Exit");
        assert_eq!(exit_with_save.label(), "Save");
    }
}
