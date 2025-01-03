use copypasta::{ClipboardContext, ClipboardProvider};
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use crate::content::{Content, ContentListItem};
use crate::popup::{Popup, PopupButton};
use crate::screen::Screen;
pub struct EventHandler {
    pub should_quit: bool,
    screen: Rc<RefCell<Screen>>,
    popup: Rc<RefCell<Popup>>,
    content: Rc<RefCell<Content>>,
    output_dir: String,
    file_name: String,
}

impl EventHandler {
    pub fn new(
        screen: Rc<RefCell<Screen>>,
        popup: Rc<RefCell<Popup>>,
        content: Rc<RefCell<Content>>,
        output_dir: String,
        file_name: String,
    ) -> Self {
        Self {
            should_quit: false,
            screen,
            popup,
            content,
            output_dir,
            file_name,
        }
    }

    pub fn listen_for_keyboard_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(250))? {
            if let event::Event::Key(key) = event::read()? {
                if self.content.borrow().enable_insert_mode {
                    self.handle_content_input(key)?;
                } else if self.screen.borrow().show_popup {
                    self.handle_popup_events(key)?
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
            KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                if let Ok(mut ctx) = ClipboardContext::new() {
                    if let Ok(contents) = ctx.get_contents() {
                        for c in contents.chars() {
                            if c == '\n' {
                                content.handle_enter();
                            } else {
                                content.insert_char(c);
                            }
                        }
                    }
                }
            }
            KeyCode::Char(c) => content.insert_char(c),
            KeyCode::Enter => content.handle_enter(),
            KeyCode::Esc => content.toggle_insert(),
            KeyCode::Backspace => content.delete_char(),
            KeyCode::Left => content.move_cursor_left(),
            KeyCode::Right => content.move_cursor_right(),
            KeyCode::Up => content.move_cursor_up(),
            KeyCode::Down => content.move_cursor_down(),
            KeyCode::F(1) => content.scroll_to_top(),
            KeyCode::F(2) => content.scroll_to_bottom(),
            _ => {}
        }
        Ok(())
    }

    fn handle_popup_events(&mut self, key: KeyEvent) -> io::Result<()> {
        match key.code {
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
            _ => {}
        }
        Ok(())
    }

    fn save_to_file(&self) -> io::Result<()> {
        let dir_path = Path::new(&self.output_dir);
        let file_path = dir_path.join(&self.file_name);

        if !dir_path.exists() {
            fs::create_dir_all(dir_path)?;
        }
        let mut file = File::create(file_path)?;
        if let Some(project_lines) = self
            .content
            .borrow()
            .file_to_save
            .get(&ContentListItem::ProjectName)
        {
            writeln!(
                file,
                "# {}",
                project_lines
                    .first()
                    .unwrap_or(&"Documentation".to_string())
            )?;
            writeln!(file, "\nThis documentation follows the Diátaxis framework.")?;
            writeln!(file, "Learn more at https://diataxis.fr/")?;
            writeln!(file)?;
        }
        let content = self.content.borrow();
        for (section, lines) in &content.file_to_save {
            if section != &ContentListItem::ProjectName {
                writeln!(file, "## {:?}", section)?;

                for line in lines {
                    writeln!(file, "{}", line)?;
                }

                writeln!(file)?;
            }
        }
        Ok(())
    }
}
