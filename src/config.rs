use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Directory to search for conflicts and duplicates
    #[serde(default = "default_working_directory")]
    pub working_directory: PathBuf,
    /// Command to use for moving files to trash
    #[serde(default = "default_trash_command")]
    pub trash_command: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            working_directory: default_working_directory(),
            trash_command: default_trash_command(),
        }
    }
}

impl Config {
    /// Get the configuration directory path
    pub fn get_config_dir() -> Result<PathBuf> {
        dirs::config_dir()
            .map(|dir| dir.join("resolvething"))
            .context("Could not determine config directory")
    }

    /// Get the configuration file path
    pub fn get_config_path() -> Result<PathBuf> {
        Ok(Self::get_config_dir()?.join("config.toml"))
    }

    /// Load configuration from disk, creating default config if it doesn't exist
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        if !config_path.exists() {
            Self::create_default_config()?;
        }
        let toml_content =
            std::fs::read_to_string(&config_path).context("Failed to read config file")?;
        toml::from_str(&toml_content).context("Failed to parse config file")
    }

    /// Save configuration to disk
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        let config_dir = Self::get_config_dir()?;
        if !config_dir.exists() {
            std::fs::create_dir_all(&config_dir).context("Failed to create config directory")?;
        }
        let toml_content = toml::to_string(self).context("Failed to serialize config")?;
        std::fs::write(&config_path, toml_content).context("Failed to write config file")?;
        Ok(())
    }

    /// Create a default configuration file
    pub fn create_default_config() -> Result<()> {
        let config = Self::default();
        config.save()
    }
}

fn default_working_directory() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("wiki/vimwiki")
}

fn default_trash_command() -> String {
    "trash".to_string()
}
