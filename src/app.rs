use crate::{config::Config, conflict::ConflictFinder, duplicates::FclonesRunner};
use anyhow::{Context, Result, bail};

/// Required external dependencies for the application
const REQUIRED_COMMANDS: &[&str] = &["fclones", "fzf", "trash", "bat"];

/// File types to scan for conflicts
const CONFLICT_FILE_TYPES: &[&str] = &["md", "json"];

/// Main application struct that coordinates conflict and duplicate resolution
pub struct App {
    config: Config,
}

impl App {
    /// Create a new App instance, checking dependencies and loading configuration
    pub fn new() -> Result<Self> {
        Self::check_dependencies()?;
        let config = Config::load().context("Failed to load configuration")?;
        Ok(Self { config })
    }

    /// Check if a command is installed and available in PATH
    fn check_command(command: &str) -> bool {
        if which::which(command).is_err() {
            eprintln!("Command '{}' not found. Please install.", command);
            return false;
        }
        true
    }

    /// Check that all required external dependencies are installed
    fn check_dependencies() -> Result<()> {
        let missing_commands: Vec<String> = REQUIRED_COMMANDS
            .iter()
            .filter(|&&command| !Self::check_command(command))
            .map(|&s| s.to_string())
            .collect();

        if !missing_commands.is_empty() {
            bail!("Missing required dependencies: {:?}", missing_commands);
        }
        Ok(())
    }

    /// Run duplicate file detection and resolution
    pub fn run_duplicate(&self) -> Result<()> {
        eprintln!("searching for duplicates");
        let mut runner = FclonesRunner::new();
        runner.run_recursively(&self.config.working_directory.to_string_lossy())?;
        runner.duplicate_groups.iter().for_each(|group| {
            if let Some(choice) = group.choose() {
                group
                    .keep_only(choice, &self.config)
                    .unwrap_or_else(|e| eprintln!("Error keeping file: {}", e));
            }
        });
        Ok(())
    }

    /// Run conflict file detection and resolution
    pub fn run_conflicts(&self) -> Result<()> {
        eprintln!("searching for conflicts");
        let mut finder =
            ConflictFinder::new(self.config.working_directory.to_string_lossy().to_string());

        for file_type in CONFLICT_FILE_TYPES {
            finder.find_conflicts(file_type)?;
        }

        finder.print_conflicts();
        for conflict in finder.conflicts {
            if let Err(e) = conflict.handle_conflict(&self.config) {
                eprintln!("Error handling conflict: {}", e);
            }
        }
        Ok(())
    }

    /// Run both duplicate and conflict resolution
    pub fn run_all(&self) -> Result<()> {
        self.run_duplicate()?;
        self.run_conflicts()?;
        Ok(())
    }
}
