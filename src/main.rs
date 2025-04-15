use clap::Parser;
use resolvething::{
    cli::{Cli, Commands},
    diff::VimDiff,
    duplicates::FdupesRunner,
};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Dupes) => {
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
        Some(Commands::All) => {
            println!("Running the 'all' command");
        }
        Some(Commands::Test) => {
            println!("Running the 'test' command");
            VimDiff::diff(
                "/home/benjamin/test/diff/test1.txt",
                "/home/benjamin/test/diff/test2.txt",
            );
        }
        None => {
            println!("Hello, world!");
        }
    }
}
