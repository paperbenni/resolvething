use std::process::{Command, Stdio};

pub struct VimDiff;

impl VimDiff {
    pub fn diff(file1: &str, file2: &str) {
        Command::new("nvim")
            .arg("-d")
            .arg(file1)
            .arg(file2)
            //TODO: figure out if this is needed
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to start neovim");
    }
}
