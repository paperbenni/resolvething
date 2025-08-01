use std::process::Command;
use std::str;

use crate::ResolvethingError;
use crate::config::Config;
use crate::fzf::Fzf;
use crate::sync_conflict_regex;
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
        let conflict_regex = sync_conflict_regex();
        if conflict_regex.is_match(file) {
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

    pub fn run_recursively(&mut self, directory: &str) -> Result<(), ResolvethingError> {
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
            .map_err(|e| {
                ResolvethingError::Duplicate(format!("Failed to execute fclones: {}", e))
            })?;

        if !output.status.success() {
            return Err(ResolvethingError::Duplicate(format!(
                "fclones failed with status: {}",
                output.status
            )));
        }

        let stdout = str::from_utf8(&output.stdout).map_err(|e| {
            ResolvethingError::Duplicate(format!("Failed to parse fclones output: {}", e))
        })?;

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
    /// use resolvething::duplicates::Duplicate;
    /// use resolvething::config::Config;
    ///
    /// let duplicate = Duplicate::new(vec![
    ///     "/path/to/file1.txt".to_string(),
    ///     "/path/to/file2.txt".to_string(),
    /// ]);
    /// let config = Config::default();
    /// duplicate.keep_only("/path/to/file1.txt".to_string(), &config);
    /// ```
    pub fn keep_only(&self, keep: String, config: &Config) {
        for file in &self.files {
            if file.path != keep {
                Trash::trash(&file.path, config);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_file_type_regular() {
        assert!(matches!(
            SyncThingFile::get_file_type("document.txt"),
            SyncThingFileType::Regular
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("image.jpg"),
            SyncThingFileType::Regular
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("data.csv"),
            SyncThingFileType::Regular
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("file.with.dots.txt"),
            SyncThingFileType::Regular
        ));
    }

    #[test]
    fn test_get_file_type_conflict() {
        assert!(matches!(
            SyncThingFile::get_file_type("document.txt.sync-conflict-20240101-123456"),
            SyncThingFileType::StConflict
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("image.sync-conflict-ABC123-XYZ789.jpg"),
            SyncThingFileType::StConflict
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("data.sync-conflict-12345"),
            SyncThingFileType::StConflict
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("file.sync-conflict-A1B2-C3D4.txt"),
            SyncThingFileType::StConflict
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("wohnen/Umzug.sync-conflict-20250412-111252-VNNIL2P.md"),
            SyncThingFileType::StConflict
        ));
    }

    #[test]
    fn test_get_file_type_orig() {
        assert!(matches!(
            SyncThingFile::get_file_type("document.txt.orig"),
            SyncThingFileType::OrigFile
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("image.jpg.orig"),
            SyncThingFileType::OrigFile
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("data.csv.orig"),
            SyncThingFileType::OrigFile
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("file.orig"),
            SyncThingFileType::OrigFile
        ));
    }

    #[test]
    fn test_get_file_type_tmp() {
        assert!(matches!(
            SyncThingFile::get_file_type("document.txt.tmp"),
            SyncThingFileType::TmpFile
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("image.jpg.tmp"),
            SyncThingFileType::TmpFile
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("data.csv.tmp"),
            SyncThingFileType::TmpFile
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("file.tmp"),
            SyncThingFileType::TmpFile
        ));
    }

    #[test]
    fn test_get_file_type_priority() {
        // Test that .sync-conflict- takes priority over .orig and .tmp
        assert!(matches!(
            SyncThingFile::get_file_type("document.txt.sync-conflict-12345.orig"),
            SyncThingFileType::StConflict
        ));
        assert!(matches!(
            SyncThingFile::get_file_type("document.txt.sync-conflict-12345.tmp"),
            SyncThingFileType::StConflict
        ));

        // Test that .tmp takes priority over .orig since .tmp is checked first
        assert!(matches!(
            SyncThingFile::get_file_type("document.txt.orig.tmp"),
            SyncThingFileType::TmpFile
        ));
    }

    #[test]
    fn test_syncthing_file_new() {
        let file = SyncThingFile::new("document.txt".to_string());
        assert_eq!(file.path, "document.txt");
        assert!(matches!(file.file_type, SyncThingFileType::Regular));

        let file = SyncThingFile::new("document.txt.sync-conflict-12345".to_string());
        assert_eq!(file.path, "document.txt.sync-conflict-12345");
        assert!(matches!(file.file_type, SyncThingFileType::StConflict));

        let file = SyncThingFile::new("document.txt.orig".to_string());
        assert_eq!(file.path, "document.txt.orig");
        assert!(matches!(file.file_type, SyncThingFileType::OrigFile));

        let file = SyncThingFile::new("document.txt.tmp".to_string());
        assert_eq!(file.path, "document.txt.tmp");
        assert!(matches!(file.file_type, SyncThingFileType::TmpFile));
    }

    #[test]
    fn test_duplicate_new() {
        let duplicate = Duplicate::new(vec![
            "file1.txt".to_string(),
            "file2.txt.sync-conflict-12345".to_string(),
            "file3.txt.orig".to_string(),
        ]);

        assert_eq!(duplicate.files.len(), 3);
        assert_eq!(duplicate.files[0].path, "file1.txt");
        assert!(matches!(
            duplicate.files[0].file_type,
            SyncThingFileType::Regular
        ));

        assert_eq!(duplicate.files[1].path, "file2.txt.sync-conflict-12345");
        assert!(matches!(
            duplicate.files[1].file_type,
            SyncThingFileType::StConflict
        ));

        assert_eq!(duplicate.files[2].path, "file3.txt.orig");
        assert!(matches!(
            duplicate.files[2].file_type,
            SyncThingFileType::OrigFile
        ));
    }

    #[test]
    fn test_duplicate_new_empty() {
        let duplicate = Duplicate::new(vec![]);
        assert_eq!(duplicate.files.len(), 0);
    }

    #[test]
    fn test_fclones_runner_parse_output() {
        let output = r#"file1.txt
file2.txt.sync-conflict-12345

file3.txt
file4.txt.orig
file5.txt.tmp

file6.txt"#;

        let mut runner = FclonesRunner::new();
        runner.parse_output(output);

        assert_eq!(runner.duplicate_groups.len(), 3);

        // First group: file1.txt and conflict file
        assert_eq!(runner.duplicate_groups[0].files.len(), 2);
        assert_eq!(runner.duplicate_groups[0].files[0].path, "file1.txt");
        assert_eq!(
            runner.duplicate_groups[0].files[1].path,
            "file2.txt.sync-conflict-12345"
        );

        // Second group: file3.txt, .orig, and .tmp
        assert_eq!(runner.duplicate_groups[1].files.len(), 3);
        assert_eq!(runner.duplicate_groups[1].files[0].path, "file3.txt");
        assert_eq!(runner.duplicate_groups[1].files[1].path, "file4.txt.orig");
        assert_eq!(runner.duplicate_groups[1].files[2].path, "file5.txt.tmp");

        // Third group: single file
        assert_eq!(runner.duplicate_groups[2].files.len(), 1);
        assert_eq!(runner.duplicate_groups[2].files[0].path, "file6.txt");
    }

    #[test]
    fn test_fclones_runner_parse_output_empty() {
        let output = "";
        let mut runner = FclonesRunner::new();
        runner.parse_output(output);
        assert_eq!(runner.duplicate_groups.len(), 0);
    }

    #[test]
    fn test_fclones_runner_parse_output_trailing_newline() {
        let output = r#"file1.txt
file2.txt.sync-conflict-12345

file3.txt
"#;

        let mut runner = FclonesRunner::new();
        runner.parse_output(output);

        assert_eq!(runner.duplicate_groups.len(), 2);
        assert_eq!(runner.duplicate_groups[0].files.len(), 2);
        assert_eq!(runner.duplicate_groups[1].files.len(), 1);
    }

    #[test]
    fn test_fclones_runner_parse_output_no_trailing_newline() {
        let output = r#"file1.txt
file2.txt.sync-conflict-12345

file3.txt"#;

        let mut runner = FclonesRunner::new();
        runner.parse_output(output);

        assert_eq!(runner.duplicate_groups.len(), 2);
        assert_eq!(runner.duplicate_groups[0].files.len(), 2);
        assert_eq!(runner.duplicate_groups[1].files.len(), 1);
    }
}
