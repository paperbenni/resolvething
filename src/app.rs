use crate::{Result, config::Config, conflict::ConflictFinder, duplicates::FclonesRunner};

pub struct App {
    config: Config,
}

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum AppInitError {
    #[error("Missing dependencies: {0:?}")]
    MissingDependencies(Vec<String>),
}

impl App {
    pub fn new() -> Result<Self> {
        Self::check_dependencies()?;
        Ok(Self {
            config: Config::load(),
        })
    }

    /// checks if a command is installed
    pub fn check_command(command: &str) -> bool {
        if which::which(command).is_err() {
            eprintln!("Command '{}' not found. Please install.", command);
            return false;
        }
        true
    }

    pub fn check_dependencies() -> std::result::Result<(), AppInitError> {
        let commands = vec!["fclones", "fzf", "trash", "bat"];
        let missing_commands: Vec<String> = commands
            .into_iter()
            .filter(|&command| !Self::check_command(command))
            .map(|s| s.to_string())
            .collect();

        if !missing_commands.is_empty() {
            Err(AppInitError::MissingDependencies(missing_commands))
        } else {
            Ok(())
        }
    }

    pub fn run_duplicate(&self) -> Result<()> {
        eprintln!("searching for duplicates");
        let mut runner = FclonesRunner::new();
        runner.run_recursively(&self.config.working_directory.to_string_lossy())?;
        runner.duplicate_groups.iter().for_each(|group| {
            if let Some(choice) = group.choose() {
                group.keep_only(choice, &self.config);
            }
        });
        Ok(())
    }

    pub fn run_conflicts(&self) -> Result<()> {
        eprintln!("searching for conflicts");
        let mut finder =
            ConflictFinder::new(self.config.working_directory.to_string_lossy().to_string());
        finder.find_conflicts("md");
        finder.find_conflicts("json");
        finder.print_conflicts();
        for conflict in finder.conflicts {
            conflict.handle_conflict(&self.config);
        }
        Ok(())
    }

    pub fn run_all(&self) -> Result<()> {
        self.run_duplicate()?;
        self.run_conflicts()?;
        Ok(())
    }
}
