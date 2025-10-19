use crate::config::Config;
use anyhow::{Context, Result};

pub struct Trash;

impl Trash {
    pub fn trash(file: &str, config: &Config) -> Result<()> {
        let output = std::process::Command::new(&config.trash_command)
            .arg(file)
            .output()
            .context("Failed to execute trash command")?;

        if output.status.success() {
            println!("Removed: {}", file);
            Ok(())
        } else {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Failed to remove {}: {}", file, error_msg)
        }
    }
}
