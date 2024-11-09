use crossterm::event::{self, KeyCode, KeyEvent};
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use crate::content::Content;
use crate::popup::{Popup, PopupButton};
use crate::screen::Screen;
pub struct EventHandler {
    pub should_quit: bool,
    screen: Rc<RefCell<Screen>>,
    popup: Rc<RefCell<Popup>>,
    content: Rc<RefCell<Content>>,
}

impl EventHandler {
    pub fn new(
        screen: Rc<RefCell<Screen>>,
        popup: Rc<RefCell<Popup>>,
        content: Rc<RefCell<Content>>,
    ) -> Self {
        Self {
            should_quit: false,
            screen,
            popup,
            content,
        }
    }

    pub fn listen_for_keyboard_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(250))? {
            if let event::Event::Key(key) = event::read()? {
                if self.content.borrow().enable_insert_mode {
                    self.handle_content_input(key)?;
                } else {
                    self.handle_navigation_input(key)?;
                }
            }
        }
        Ok(())
    }

    pub fn handle_navigation_input(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Char('s') => {
                self.save_to_file()?;
            }
            KeyCode::Char('i') => self.content.borrow_mut().toggle_insert(),
            KeyCode::Esc => {
                self.screen.borrow_mut().toggle_popup();
            }
            KeyCode::Right => {
                self.popup.borrow_mut().next_button();
            }
            KeyCode::Left => {
                self.popup.borrow_mut().previous_button();
            }
            KeyCode::Enter => match self.popup.borrow().select_button() {
                PopupButton::Cancel => {
                    self.screen.borrow_mut().toggle_popup();
                }
                PopupButton::ExitWithoutSaving => {
                    self.should_quit = true;
                }
                PopupButton::ExitWithSave => {
                    self.save_to_file()?;
                    self.should_quit = true;
                }
            },
            KeyCode::Down => self
                .screen
                .borrow_mut()
                .next(&mut self.content.borrow_mut()),
            KeyCode::Up => self
                .screen
                .borrow_mut()
                .previous(&mut self.content.borrow_mut()),
            _ => {}
        }
        Ok(())
    }

    fn handle_content_input(&mut self, key: KeyEvent) -> io::Result<()> {
        let mut content = self.content.borrow_mut();

        match key.code {
            KeyCode::Char(c) => {
                content.content_input.push(c);
                content.cursor_index_x += 1;
            }
            KeyCode::Backspace => {
                if content.cursor_index_x > 0 {
                    content.content_input.pop();
                    content.cursor_index_x -= 1;
                }
            }
            KeyCode::Enter => {
                content.content_input.push('\n');
                content.cursor_index_y += 1;
                content.cursor_index_x = 0;
            }
            KeyCode::Esc => {
                content.toggle_insert();
            }
            KeyCode::Left => {
                if content.cursor_index_x > 0 {
                    content.cursor_index_x -= 1;
                }
            }
            KeyCode::Right => {
                if content.cursor_index_x < content.content_input.len() {
                    content.cursor_index_x += 1;
                }
            }
            KeyCode::Up => {
                if content.cursor_index_y > 0 {
                    content.cursor_index_y -= 1;
                }
            }
            KeyCode::Down => {
                content.cursor_index_y += 1;
            }
            KeyCode::Tab => {
                content.cursor_index_x += 4;
            }
            _ => {}
        }
        Ok(())
    }

    fn save_to_file(&self) -> io::Result<()> {
        let dir_path = Path::new("output");
        let file_path = dir_path.join("README.md");

        if !dir_path.exists() {
            fs::create_dir_all(dir_path)?;
        }
        let mut file = File::create(file_path)?;
        let content = self.content.borrow();
        for section in content.file_to_save.iter() {
            writeln!(file, "##\n{}", section.1)?;
        }

        println!("Your input has been saved to output/README.md");
        Ok(())
    }
}
