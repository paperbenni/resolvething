use std::path::PathBuf;

use regex::Regex;
use walkdir::WalkDir;

use crate::diff::VimDiff;

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
        return path.exists()
            && path.is_file()
            && { std::fs::metadata(&path).ok().unwrap().len() < 1_000_000 }
            && {
                if let Ok(content) = std::fs::read(&path) {
                    // Simple heuristic: check if file contains null bytes (common in binary files)
                    !content.contains(&0)
                } else {
                    false
                }
            };
    }

    pub fn is_valid(&self) -> bool {
        self.originalfile != self.modifiedfile
            && Conflict::file_is_valid(&self.originalfile)
            && Conflict::file_is_valid(&self.modifiedfile)
    }

    pub fn handle_conflict(&self) {
        if !self.is_valid() {
            return;
        }

        if !PathBuf::from(&self.modifiedfile).exists() && PathBuf::from(&self.originalfile).exists()
        {
            println!("conflict already resolved:");
            self.print();
            println!("");
            return;
        }
        VimDiff::diff(&self.modifiedfile, &self.originalfile);

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
            println!("I would have deleted {} here", &self.modifiedfile);
            // TODO: uncomment after enough testing
            // Trash::trash(&self.modifiedfile);
        }
    }
}

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

    pub fn find_conflicts(&mut self) {
        // walkdir across directory, find
        // files which match the regex .*\.sync-conflict-[A-Z0-9-]*\.md$
        let regex = Regex::new(r".*\.sync-conflict-[A-Z0-9-]*\.md$").unwrap();
        let replaceexp = Regex::new(r"\.sync-conflict-[A-Z0-9-]*\.md$").unwrap();
        for entry in WalkDir::new(&self.directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() && regex.is_match(entry.path().to_str().unwrap()) {
                let originalfile = replaceexp
                    .replace_all(entry.path().to_str().unwrap(), ".md")
                    .to_string();
                let modifiedfile = entry.path().to_str().unwrap().to_string();

                self.conflicts.push(Conflict {
                    originalfile,
                    modifiedfile,
                });
            }
        }
    }

    pub fn print_conflicts(&self) {
        for conflict in &self.conflicts {
            conflict.print();
        }
    }
}
