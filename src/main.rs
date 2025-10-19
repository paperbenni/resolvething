use anyhow::Result;
use clap::Parser;
use resolvething::{
    app::App,
    cli::{Cli, Commands},
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let app = App::new()?;

    if let Some(command) = cli.command {
        match command {
            Commands::Dupes => app.run_duplicate()?,
            Commands::Conflicts => app.run_conflicts()?,
            Commands::All => app.run_all()?,
        }
    } else {
        app.run_all()?;
    }

    Ok(())
}
