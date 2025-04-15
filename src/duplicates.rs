use std::process::Command;

use std::str;

use crate::fzf::Fzf;
use crate::trash::Trash;

pub struct FdupesRunner {
    pub duplicate_groups: Vec<Duplicate>,
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
                    self.duplicate_groups.push(Duplicate::new(current_group));
                    current_group = Vec::new();
                }
            } else {
                current_group.push(line.to_string());
            }
        }

        if !current_group.is_empty() {
            self.duplicate_groups.push(Duplicate::new(current_group));
        }
    }
}

pub struct Duplicate {
    pub files: Vec<String>,
}

impl Duplicate {
    pub fn new(files: Vec<String>) -> Self {
        Duplicate { files }
    }

    pub fn choose(&self) -> Option<String> {
        let options = self.files.clone();
        let choice = Fzf::select(options);
        if let Some(selected) = choice {
            println!("Selected: {}", selected);
            Some(selected)
        } else {
            println!("No selection made");
            None
        }
    }

    /// Keeps the specified file and moves other duplicates to the trash.
    ///
    /// # Arguments
    ///
    /// * `keep` - The file path to keep. Other duplicates will be trashed.
    ///
    /// # Example
    ///
    /// ```
    /// let duplicate = Duplicate::new(vec![
    ///     "/path/to/file1.txt".to_string(),
    ///     "/path/to/file2.txt".to_string(),
    /// ]);
    /// duplicate.keep_only("/path/to/file1.txt".to_string());
    /// ```
    pub fn keep_only(&self, keep: String) {
        for file in &self.files {
            if *file != keep {
                Trash::trash(file);
            }
        }
    }
}
