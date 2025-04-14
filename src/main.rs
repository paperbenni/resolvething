use clap::{Parser, Subcommand};
use std::str;
use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Dupes,
    All,
}

pub struct FdupesRunner {
    pub duplicate_groups: Vec<Vec<String>>,
}

pub struct Fzf;

impl Fzf {
    /// Launches an fzf process to allow the user to select an item from the provided vector.
    ///
    /// # Arguments
    ///
    /// * `items` - A vector of strings representing the items to choose from.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The selected item as a `String`, or `None` if no selection was made.
    pub fn select(items: Vec<String>) -> Option<String> {
        let mut child = Command::new("fzf")
            .arg("--preview")
            .arg("bat --style=plain --paging=never --color=always {}")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start fzf process");

        {
            let stdin = child.stdin.as_mut().expect("Failed to open stdin");
            for item in items {
                writeln!(stdin, "{}", item).expect("Failed to write to stdin");
            }
        }

        let output = child.wait_with_output().expect("Failed to read fzf output");

        if output.status.success() {
            let selected = BufReader::new(output.stdout.as_slice())
                .lines()
                .next()
                .unwrap_or_else(|| Ok(String::new()))
                .unwrap();
            if !selected.is_empty() {
                return Some(selected);
            }
        }

        None
    }
}

impl FdupesRunner {
    pub fn new() -> Self {
        FdupesRunner {
            duplicate_groups: Vec::new(),
        }
    }

    pub fn run_recursively(&mut self, directory: &str) -> Result<(), String> {
        let output = Command::new("fdupes")
            .arg("-r")
            .arg(directory)
            .output()
            .map_err(|e| format!("Failed to execute fdupes: {}", e))?;

        if !output.status.success() {
            return Err(format!("fdupes failed with status: {}", output.status));
        }

        let stdout = str::from_utf8(&output.stdout)
            .map_err(|e| format!("Failed to parse fdupes output: {}", e))?;

        self.parse_output(stdout);
        Ok(())
    }

    fn parse_output(&mut self, output: &str) {
        let mut current_group = Vec::new();

        for line in output.lines() {
            if line.is_empty() {
                if !current_group.is_empty() {
                    self.duplicate_groups.push(current_group);
                    current_group = Vec::new();
                }
            } else {
                current_group.push(line.to_string());
            }
        }

        if !current_group.is_empty() {
            self.duplicate_groups.push(current_group);
        }
    }
}

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
                        let selected  = Fzf::select(options);
                        if let Some(selected) = selected {
                            println!("Selected: {}", selected);
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
