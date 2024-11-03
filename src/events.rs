use crossterm::event::{self, KeyCode};
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use crate::screen::Screen;
pub struct EventHandler {
    pub should_quit: bool,
    pub content: Vec<String>,
    screen: Rc<RefCell<Screen>>,
}

impl EventHandler {
    pub fn new(screen: Rc<RefCell<Screen>>) -> Self {
        Self {
            should_quit: false,
            content: Vec::new(),
            screen
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
