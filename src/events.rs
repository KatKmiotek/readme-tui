use crossterm::event::{self, KeyCode};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::time::Duration;

pub struct EventHandler {
    pub should_quit: bool,
    pub content: Vec<String>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            content: Vec::new(),
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
