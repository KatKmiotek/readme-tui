use crate::{content::Content, events::EventHandler, popup::Popup, screen::Screen, CliConfig};
use color_eyre::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
    cell::RefCell,
    io::{stdout, Stdout},
    rc::Rc,
};
pub struct App {
    event_handler: EventHandler,
    screen: Rc<RefCell<Screen>>,
    popup: Rc<RefCell<Popup>>,
    content: Rc<RefCell<Content>>,
}

impl App {
    pub fn new(cli_config: CliConfig) -> App {
        let screen = Rc::new(RefCell::new(Screen::new()));
        let popup = Rc::new(RefCell::new(Popup::new()));
        let content = Rc::new(RefCell::new(Content::new()));
        let event_handler = EventHandler::new(
            Rc::clone(&screen),
            Rc::clone(&popup),
            Rc::clone(&content),
            cli_config.output_dir.clone(),
            cli_config.file_name.clone(),
        );
        Self {
            event_handler,
            screen,
            popup,
            content,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(EnableMouseCapture)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_app(&mut terminal);

        terminal::disable_raw_mode()?;
        terminal.backend_mut().execute(LeaveAlternateScreen)?;
        terminal.backend_mut().execute(DisableMouseCapture)?;

        res
    }

    fn run_app(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| {
                self.screen.borrow_mut().get_layout(
                    f,
                    &mut self.popup.borrow_mut(),
                    &mut self.content.borrow_mut(),
                )
            })?;
            self.event_handler.listen_for_keyboard_events()?;

            if self.event_handler.should_quit {
                break;
            }
        }
        Ok(())
    }
}
