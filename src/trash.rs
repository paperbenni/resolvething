pub struct Trash;

impl Trash {
    pub fn trash(file: &str) {
        let output = std::process::Command::new("trash").arg(&file).output();
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
