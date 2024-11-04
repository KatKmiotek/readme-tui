use crate::{events::EventHandler, popup::Popup, screen::Screen};
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
}

impl App {
    pub fn new() -> App {
        let screen = Rc::new(RefCell::new(Screen::new()));
        let popup = Rc::new(RefCell::new(Popup::new()));
        let event_handler = EventHandler::new(Rc::clone(&screen), Rc::clone(&popup));
        Self {
            event_handler,
            screen,
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
            // terminal.draw(|f| ui(f))?;
            terminal.draw(|f| self.screen.borrow_mut().get_layout(f))?;
            self.event_handler.listen_for_keyboard_events()?;

            if self.event_handler.should_quit {
                break;
            }
        }
        Ok(())
    }
}
