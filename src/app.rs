use crate::{config::Config, conflict::ConflictFinder, duplicates::FclonesRunner};

pub struct App {
    config: Config,
}

impl App {
    pub fn new() -> Self {
        Self::check_dependencies();
        Self {
            config: Config::load(),
        }
    }

    /// checks if a command is installed
    pub fn check_command(command: &str) -> bool {
        if which::which(command).is_err() {
            eprintln!("Command '{}' not found. Please install.", command);
            return false;
        }
        true
    }

    pub fn check_dependencies() {
        let commands = vec!["fclones", "fzf", "trash", "bat"];
        let mut any_missing = false;
        for command in commands {
            if !Self::check_command(command) {
                any_missing = true;
            }
        }
        if any_missing {
            eprintln!("Some dependencies are missing. Please install them to use this program.");
            //TODO: this is horrible, do it better
            std::process::exit(1);
        }
    }

    pub fn run_duplicate(&self) {
        eprintln!("searching for duplicates");
        let mut runner = FclonesRunner::new();
        runner
            .run_recursively(&self.config.working_directory.to_string_lossy())
            .unwrap();
        runner.duplicate_groups.iter().for_each(|group| {
            if let Some(choice) = group.choose() {
                group.keep_only(choice, &self.config);
            }
        });
    }

    pub fn run_conflicts(&self) {
        eprintln!("searching for conflicts");
        let mut finder =
            ConflictFinder::new(self.config.working_directory.to_string_lossy().to_string());
        finder.find_conflicts("md");
        finder.find_conflicts("json");
        finder.print_conflicts();
        for conflict in finder.conflicts {
            conflict.handle_conflict(&self.config);
        }
    }

    pub fn run_all(&self) {
        self.run_duplicate();
        self.run_conflicts();
    }
}
