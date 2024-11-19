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

    #[arg(short = 'o', long, default_value = "txtui-output")]
    output_dir: String,

    #[arg(short = 'f', long, default_value = "README.md")]
    file_name: String,
}

pub struct CliConfig {
    pub output_dir: String,
    pub file_name: String,
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    let output_dir = cli.output_dir;
    let file_name = cli.file_name;

    if cli.version {
        println!("txtui version {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    color_eyre::install()?;
    let mut app = App::new(CliConfig {
        output_dir,
        file_name,
    });
    app.run()?;
    Ok(())
}
