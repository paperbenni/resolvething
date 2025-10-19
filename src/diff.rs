use anyhow::{Context, Result};
use std::process::{Command, Stdio};

pub struct VimDiff;

impl VimDiff {
    pub fn diff(file1: &str, file2: &str) -> Result<()> {
        Command::new("nvim")
            .arg("-d")
            .arg(file1)
            .arg(file2)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .context("Failed to start neovim")?
            .wait()
            .context("neovim exited with error")?;
        Ok(())
    }
}
