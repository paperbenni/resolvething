use anyhow::{Context, Result};
use std::{
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
};

pub struct Fzf;

impl Fzf {
    /// Launches an fzf process to allow the user to select an item from the provided vector.
    ///
    /// # Arguments
    ///
    /// * `items` - A vector of strings representing the items to choose from.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The selected item as a `String`, or `None` if no selection was made.
    pub fn select(items: Vec<String>) -> Option<String> {
        Self::select_internal(items).ok().flatten()
    }

    fn select_internal(items: Vec<String>) -> Result<Option<String>> {
        let mut child = Command::new("fzf")
            .arg("--preview")
            .arg("bat --style=plain --paging=never --color=always {}")
            .arg("--preview-window")
            .arg(if termsize::get().is_some_and(|size| size.cols < 80) {
                "down:50%"
            } else {
                "right:50%"
            })
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .context("Failed to start fzf process")?;

        {
            let stdin = child.stdin.as_mut().context("Failed to open fzf stdin")?;
            for item in items {
                writeln!(stdin, "{}", item).context("Failed to write to fzf stdin")?;
            }
        }

        let output = child
            .wait_with_output()
            .context("Failed to read fzf output")?;

        if output.status.success() {
            let selected = BufReader::new(output.stdout.as_slice())
                .lines()
                .next()
                .transpose()
                .context("Failed to read fzf selection")?
                .unwrap_or_default();
            if !selected.is_empty() {
                return Ok(Some(selected));
            }
        }

        Ok(None)
    }
}
