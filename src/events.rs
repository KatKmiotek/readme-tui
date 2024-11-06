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
                if self.screen.borrow().enable_insert_mode {
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
            KeyCode::Char('i') => self.screen.borrow_mut().enable_insert(),
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
            KeyCode::Down => self.screen.borrow_mut().next(),
            KeyCode::Up => self.screen.borrow_mut().previous(),
            _ => {}
        }
        Ok(())
    }

    fn handle_content_input(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
            KeyCode::Char(c) => {
                self.content.borrow_mut().content_input.push(c);
            }
            KeyCode::Backspace => {
                self.content.borrow_mut().content_input.pop();
            }
            // KeyCode::Enter => {
            //     self.screen.borrow_mut().enable_insert();
            // }
            KeyCode::Esc => {
                self.screen.borrow_mut().enable_insert();
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
        writeln!(file, "{}", self.content.borrow().content_input.as_str())?;

        println!("Your input has been saved to output/README.md");
        Ok(())
    }
}
