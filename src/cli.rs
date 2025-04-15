use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Find duplicate files, select which file to keep, delete the rest
    Dupes,
    /// Merge files wich have been modified on multiple devices
    Conflicts,
    /// Run all commands
    All,
}
