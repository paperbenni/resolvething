use clap::Parser;
use resolvething::{
    app::App,
    cli::{Cli, Commands},
};

fn main() {
    let app = App::new();
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        match command {
            Commands::Dupes => app.run_duplicate(),
            Commands::Conflicts => app.run_conflicts(),
            Commands::All => app.run_all(),
        }
    } else {
        println!("Nothing happening");
    }
}
