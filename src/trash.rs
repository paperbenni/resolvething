use crate::config::Config;

pub struct Trash;

impl Trash {
    pub fn trash(file: &str, config: &Config) {
        let output = std::process::Command::new(&config.trash_command)
            .arg(&file)
            .output();
        match output {
            Ok(output) if output.status.success() => {
                println!("Removed: {}", file);
            }
            Ok(output) => {
                eprintln!(
                    "Failed to remove {}: {}",
                    file,
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            Err(e) => {
                eprintln!("Error executing trash command for {}: {}", file, e);
            }
        }
    }
}
