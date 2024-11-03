mod app;
mod events;
mod ui;
use app::App;
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app = App::new();
    app.run()?;
    Ok(())
}
