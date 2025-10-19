pub mod app;
pub mod cli;
pub mod config;
pub mod conflict;
pub mod diff;
pub mod duplicates;
pub mod fzf;
pub mod trash;

use regex::Regex;

pub type Result<T> = anyhow::Result<T>;

/// Returns a regex that matches Syncthing conflict files
///
/// Conflict files follow the pattern: `.*\.sync-conflict-[A-Z0-9-]*(\..*)?$`
pub fn sync_conflict_regex() -> Regex {
    Regex::new(r".*\.sync-conflict-[A-Z0-9-]*(\..*)?$")
        .expect("Invalid regex pattern for sync conflict")
}

/// Returns a regex that matches Syncthing conflict files for a specific file type
///
/// # Example
///
/// ```
/// use resolvething::sync_conflict_regex_for_type;
/// let regex = sync_conflict_regex_for_type("txt");
/// assert!(regex.is_match("document.txt.sync-conflict-20240101-123456.txt"));
/// assert!(!regex.is_match("document.txt.sync-conflict-20240101-123456.md"));
/// ```
pub fn sync_conflict_regex_for_type(file_type: &str) -> Regex {
    Regex::new(&format!(r".*\.sync-conflict-[A-Z0-9-]*\.{}$", file_type))
        .expect("Invalid regex pattern for sync conflict with file type")
}

/// Returns a regex that replaces the conflict suffix with the original file type
pub fn sync_conflict_replace_regex_for_type(file_type: &str) -> Regex {
    Regex::new(&format!(r"\.sync-conflict-[A-Z0-9-]*\.{}$", file_type))
        .expect("Invalid regex pattern for replacing sync conflict suffix")
}
