use std::process::Command;

use std::str;

pub struct FdupesRunner {
    pub duplicate_groups: Vec<Vec<String>>,
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
