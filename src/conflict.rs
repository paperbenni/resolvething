use std::path::PathBuf;

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::{
    config::Config, diff::VimDiff, sync_conflict_regex_for_type,
    sync_conflict_replace_regex_for_type, trash::Trash,
};

/// Maximum file size (in bytes) to process for conflict resolution
const MAX_FILE_SIZE: u64 = 1_000_000;

/// Name of the Syncthing versions directory to skip
const STVERSIONS_DIR: &str = ".stversions";

/// Represents a conflict between an original file and a modified version
pub struct Conflict {
    pub originalfile: String,
    pub modifiedfile: String,
}

impl Conflict {
    pub fn new(originalfile: String, modifiedfile: String) -> Self {
        Conflict {
            originalfile,
            modifiedfile,
        }
    }
    pub fn print(&self) {
        println!("Original file: {}", self.originalfile);
        println!("Modified file: {}", self.modifiedfile);
    }

    pub fn file_is_valid(file: &str) -> bool {
        let path = PathBuf::from(file);
        path.exists()
            && path.is_file()
            && std::fs::metadata(&path)
                .ok()
                .map(|m| m.len() < MAX_FILE_SIZE)
                .unwrap_or(false)
            && std::fs::read(&path)
                .ok()
                .map(|content| !content.contains(&0))
                .unwrap_or(false)
    }

    pub fn is_valid(&self) -> bool {
        self.originalfile != self.modifiedfile
            && Conflict::file_is_valid(&self.originalfile)
            && Conflict::file_is_valid(&self.modifiedfile)
    }

    pub fn handle_conflict(&self, config: &Config) -> Result<()> {
        if !self.is_valid() {
            return Ok(());
        }

        if !PathBuf::from(&self.modifiedfile).exists() && PathBuf::from(&self.originalfile).exists()
        {
            println!("conflict already resolved:");
            self.print();
            println!();
            return Ok(());
        }
        VimDiff::diff(&self.modifiedfile, &self.originalfile)?;

        let resolved = if let (Ok(original_content), Ok(modified_content)) = (
            std::fs::read_to_string(&self.originalfile),
            std::fs::read_to_string(&self.modifiedfile),
        ) {
            // Compare the contents
            original_content == modified_content
        } else {
            // If we can't read either file, they're not resolved
            false
        };
        if resolved {
            Trash::trash(&self.modifiedfile, config)?;
        }
        Ok(())
    }
}

/// Finds and manages Syncthing conflict files in a directory
pub struct ConflictFinder {
    pub directory: String,
    pub conflicts: Vec<Conflict>,
}

impl ConflictFinder {
    pub fn new(directory: String) -> Self {
        ConflictFinder {
            directory,
            conflicts: Vec::new(),
        }
    }

    pub fn find_conflicts(&mut self, file_type: &str) -> Result<()> {
        // walkdir across directory, find
        let regex = sync_conflict_regex_for_type(file_type);
        let replaceexp = sync_conflict_replace_regex_for_type(file_type);
        for entry in WalkDir::new(&self.directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry
                .path()
                .components()
                .filter_map(|comp| comp.as_os_str().to_str())
                .any(|segment| segment == STVERSIONS_DIR)
            {
                println!("skipping stversions file {}", entry.path().display());
                continue;
            }
            if entry.file_type().is_file() {
                let path_str = entry
                    .path()
                    .to_str()
                    .context("Invalid UTF-8 in file path")?;

                if regex.is_match(path_str) {
                    let originalfile = replaceexp
                        .replace_all(path_str, &format!(".{}", file_type))
                        .to_string();
                    let modifiedfile = path_str.to_string();

                    self.conflicts.push(Conflict {
                        originalfile,
                        modifiedfile,
                    });
                }
            }
        }
        Ok(())
    }

    pub fn print_conflicts(&self) {
        for conflict in &self.conflicts {
            conflict.print();
        }
    }
}
