use std::path::{Path, PathBuf};

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

    pub fn is_valid(&self) -> bool {
        let original_exists = PathBuf::from(&self.originalfile).exists();
        let modified_exists = PathBuf::from(&self.modifiedfile).exists();
        self.originalfile != self.modifiedfile && original_exists && modified_exists
    }

    pub fn handle_conflict(&self) {
        // Implement conflict handling logic here
        // if file "$conflict_file" | grep -q "text"; then
        //   # Check if the file size is within the limit
        //   if [ "$(stat -c%s "$conflict_file")" -le "$MAX_SIZE" ]; then
        //     # Derive the original file name (assuming a naming convention)
        //     original_file="$(
        //         sed 's/\.sync-conflict-[A-Z0-9-]*\.md$/.md/' <<< "$conflict_file"
        //     )"
        //     echo "conflict_file: $conflict_file"
        //     echo "original_file: $original_file"

        //     if [ "$conflict_file"  = "$original_file" ]; then
        //       echo "conflict file and original file are the same file, wtf"
        //       echo "skipping"
        //       continue
        //     fi

        //     # Check if the original file exists
        //     if [ -f "$original_file" ]; then
        //       # Open both files in Neovim diff mode
        //       nvim -d "$conflict_file" "$original_file"

        //       # After Neovim exits, compare the files
        //       if cmp -s "$conflict_file" "$original_file"; then
        //         # If files are identical, remove the conflict file
        //         trash "$conflict_file"
        //         echo "Removed identical conflict file: $conflict_file"
        //       else
        //         echo "Files are different. Conflict file not removed: $conflict_file"
        //       fi
        //     else
        //       echo "Original file not found for: $conflict_file"
        //     fi
        //   fi
        if !self.is_valid() {
            return;
        }
        VimDiff::diff(&self.modifiedfile, &self.originalfile);
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
        for entry in WalkDir::new(&self.directory)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() && regex.is_match(entry.path().to_str().unwrap()) {
                let originalfile = entry.path().to_str().unwrap().replacen(
                    &regex.to_string()[2..regex.to_string().len() - 7],
                    "",
                    1,
                );
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
