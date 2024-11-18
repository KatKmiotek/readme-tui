mod app;
pub mod content;
mod events;
pub mod popup;
pub mod screen;
use app::App;
use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Parser)]
#[command(author, about, long_about = None)]
struct Cli {
    #[arg(short = 'v', long)]
    version: bool,
}
fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.version {
        println!("txtui version {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    color_eyre::install()?;
    let mut app = App::new();
    app.run()?;
    Ok(())
}
