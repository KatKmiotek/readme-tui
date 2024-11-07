mod app;
pub mod content;
mod events;
pub mod popup;
pub mod screen;
use app::App;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    app.run()?;
    Ok(())
}
