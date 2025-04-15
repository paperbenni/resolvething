use regex::Regex;
use walkdir::WalkDir;

pub struct Conflict {
    pub originalfile: String,
    pub modifiedfile: String,
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
            println!("Original file: {}", conflict.originalfile);
            println!("Modified file: {}", conflict.modifiedfile);
        }
    }
}
