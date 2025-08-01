pub mod app;
pub mod cli;
pub mod config;
pub mod conflict;
pub mod diff;
pub mod duplicates;
pub mod fzf;
pub mod trash;

use app::AppInitError;
use regex::Regex;

/// Returns a regex that matches Syncthing conflict files
///
/// Conflict files follow the pattern: `.*\.sync-conflict-[A-Z0-9-]*(\..*)?$`
pub fn sync_conflict_regex() -> Regex {
    Regex::new(r".*\.sync-conflict-[A-Z0-9-]*(\..*)?$").unwrap()
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
    Regex::new(&format!(r".*\.sync-conflict-[A-Z0-9-]*\.{}$", file_type)).unwrap()
}

/// Returns a regex that replaces the conflict suffix with the original file type
pub fn sync_conflict_replace_regex_for_type(file_type: &str) -> Regex {
    Regex::new(&format!(r"\.sync-conflict-[A-Z0-9-]*\.{}$", file_type)).unwrap()
}

use thiserror::Error;

/// Top-level error type for resolvething application
#[derive(Debug, Error, Clone)]
pub enum ResolvethingError {
    #[error("Application initialization error: {0}")]
    Init(#[from] AppInitError),

    #[error("Duplicate resolution error: {0}")]
    Duplicate(String),
}

pub type Result<T> = std::result::Result<T, ResolvethingError>;
