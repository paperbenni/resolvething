use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_working_directory")]
    pub working_directory: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            working_directory: default_working_directory(),
        }
    }
}

impl Config {
    pub fn get_config_dir() -> PathBuf {
        let config_dir = dirs::config_dir().unwrap().join("resolvething");
        config_dir
    }
    pub fn get_config_path() -> PathBuf {
        let config_dir = Self::get_config_dir();
        config_dir.join("config.toml")
    }

    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        if !config_path.exists() {
            Self::create_default_config();
        }
        let toml_content = std::fs::read_to_string(&config_path).unwrap();
        toml::from_str(&toml_content).unwrap()
    }

    pub fn save(&self) {
        let config_path = Self::get_config_path();
        let config_dir = Self::get_config_dir();
        if !config_dir.exists() {
            std::fs::create_dir(&config_dir).unwrap();
        }
        let toml_content = toml::to_string(self).unwrap();
        std::fs::write(&config_path, toml_content).unwrap();
    }
    pub fn create_default_config() {
        let config = Self::default();
        config.save();
    }
}

fn default_working_directory() -> PathBuf {
    dirs::home_dir().unwrap().join("wiki/vimwiki")
}
