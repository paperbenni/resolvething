use clap::Parser;
use resolvething::{
    cli::{Cli, Commands},
    config::Config,
    duplicates::FdupesRunner,
};

fn main() {
    let config = Config::load();
    println!(
        "config default working dir: {}",
        &config.working_directory.display()
    );
    return;
    let cli = Cli::parse();
    if let Some(command) = cli.command {
        match command {
            Commands::Dupes => {
                let mut runner = FdupesRunner::new();
                match runner.run_recursively("/home/benjamin/wiki/vimwiki") {
                    Ok(_) => {
                        println!("Duplicate groups found:");
                        for group in runner.duplicate_groups {
                            if let Some(choice) = group.choose() {
                                group.keep_only(choice);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
            Commands::All => {
                println!("Running the 'all' command");
            }
        }
    } else {
        println!("Nothing happening");
    }
}
