use clap::Parser;
use resolvething::{
    cli::{Cli, Commands},
    fdupes_runner::FdupesRunner,
    fzf::Fzf,
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
                        println!("{:?}", group);
                        let options = group.clone();
                        let selected = Fzf::select(options);
                        if let Some(selected) = selected {
                            println!("Selected: {}", selected);
                            for file in group {
                                if file != selected {
                                    let output =
                                        std::process::Command::new("trash").arg(&file).output();
                                    match output {
                                        Ok(output) if output.status.success() => {
                                            println!("Removed: {}", file);
                                        }
                                        Ok(output) => {
                                            eprintln!(
                                                "Failed to remove {}: {}",
                                                file,
                                                String::from_utf8_lossy(&output.stderr)
                                            );
                                        }
                                        Err(e) => {
                                            eprintln!(
                                                "Error executing trash command for {}: {}",
                                                file, e
                                            );
                                        }
                                    }
                                }
                            }
                        } else {
                            println!("No selection made.");
                            continue;
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
        None => {
            println!("Hello, world!");
        }
    }
}
