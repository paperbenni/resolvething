use regex::Regex;
use std::path::Path;
use std::process::Command;
use std::str;

use crate::config::Config;
use crate::fzf::Fzf;
use crate::trash::Trash;

pub struct FclonesRunner {
    pub duplicate_groups: Vec<Duplicate>,
}

#[derive(Clone)]
pub struct SyncThingFile {
    pub path: String,
    pub file_type: SyncThingFileType,
}

impl SyncThingFile {
    pub fn get_file_type(file: &str) -> SyncThingFileType {
        // TODO: use regex .*\.sync-conflict-[A-Z0-9-]*\..*$
        if file.contains(".sync-conflict-") {
            SyncThingFileType::StConflict
        } else if file.ends_with(".orig") {
            SyncThingFileType::OrigFile
        } else if file.ends_with(".tmp") {
            SyncThingFileType::TmpFile
        } else {
            SyncThingFileType::Regular
        }
    }

    pub fn new(path: String) -> Self {
        let filetype = Self::get_file_type(&path);
        SyncThingFile {
            path,
            file_type: filetype,
        }
    }
}

#[derive(Clone)]
pub enum SyncThingFileType {
    Regular,
    StConflict,
    OrigFile,
    TmpFile,
}

impl FclonesRunner {
    pub fn new() -> Self {
        FclonesRunner {
            duplicate_groups: Vec::new(),
        }
    }

    pub fn run_recursively(&mut self, directory: &str) -> Result<(), String> {
        let output = Command::new("fclones")
            .arg("group")
            .arg("--hidden")
            .arg(directory)
            .arg("--format")
            .arg("fdupes")
            .arg("--cache")
            .arg("--exclude")
            .arg("**/.stversions/**")
            .output()
            .map_err(|e| format!("Failed to execute fclones: {}", e))?;

        if !output.status.success() {
            return Err(format!("fclones failed with status: {}", output.status));
        }

        let stdout = str::from_utf8(&output.stdout)
            .map_err(|e| format!("Failed to parse fclones output: {}", e))?;

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
    pub files: Vec<SyncThingFile>,
}

impl Duplicate {
    pub fn new(file_paths: Vec<String>) -> Self {
        let files = file_paths
            .into_iter()
            .map(|path| SyncThingFile::new(path))
            .collect();
        Duplicate { files }
    }

    pub fn choose(&self) -> Option<String> {
        // Check if we can automatically select a file
        if let Some(auto_selected) = self.try_auto_select() {
            println!("Auto-selected file: {}", auto_selected);
            return Some(auto_selected);
        }

        // Otherwise, proceed with normal selection
        let options: Vec<String> = self.files.iter().map(|file| file.path.clone()).collect();

        let choice = Fzf::select(options);
        if let Some(selected) = choice {
            println!("Selected: {}", selected);
            Some(selected)
        } else {
            println!("No selection made");
            None
        }
    }

    /// Attempts to automatically select a file based on file types
    ///
    /// Returns the path of the selected file if auto-selection is possible,
    /// or None if user selection is needed
    fn try_auto_select(&self) -> Option<String> {
        // Count files by type

        let mut regular_files = vec![];
        let mut conflict_files = vec![];
        let mut tmp_files = vec![];
        let mut orig_files = vec![];

        for file in &self.files {
            match file.file_type {
                SyncThingFileType::Regular => regular_files.push(file),
                SyncThingFileType::StConflict => conflict_files.push(file),
                SyncThingFileType::OrigFile => orig_files.push(file),
                SyncThingFileType::TmpFile => tmp_files.push(file),
            }
        }

        if self.files.len() == 0 {
            return None;
        } else if regular_files.len() == 1 && regular_files.len() < self.files.len() {
            return Some(regular_files[0].path.clone());
        }

        // Case 2: If there are only temporary files and one conflict file
        if conflict_files.len() == 1 && tmp_files.len() + conflict_files.len() == self.files.len() {
            return Some(conflict_files[0].path.clone());
        }

        if self.files.len() == tmp_files.len() {
            return Some(tmp_files[0].path.clone());
        }

        if self.files.len() == conflict_files.len() {
            return Some(conflict_files[0].path.clone());
        }
        // No auto-selection possible
        None
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
    pub fn keep_only(&self, keep: String, config: &Config) {
        for file in &self.files {
            if file.path != keep {
                Trash::trash(&file.path, config);
            }
        }
    }
}
