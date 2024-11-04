use crossterm::event::{self, KeyCode};
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use crate::popup::{Popup, PopupButton};
use crate::screen::Screen;
pub struct EventHandler {
    pub should_quit: bool,
    pub content: Vec<String>,
    screen: Rc<RefCell<Screen>>,
    popup: Rc<RefCell<Popup>>,
}

impl EventHandler {
    pub fn new(screen: Rc<RefCell<Screen>>, popup: Rc<RefCell<Popup>>) -> Self {
        Self {
            should_quit: false,
            content: Vec::new(),
            screen,
            popup,
        }
    }

    pub fn listen_for_keyboard_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(250))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        self.should_quit = true;
                    }
                    KeyCode::Char('s') => {
                        self.save_to_file()?;
                    }
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
            }
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
        for line in &self.content {
            writeln!(file, "{}", line)?;
        }
        println!("Your input has been saved to output/README.md");
        Ok(())
    }
}
