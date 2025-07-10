use std::fs;
use std::path::{Path, PathBuf};

pub fn quarantine_duplicates(duplicate_groups: Vec<Vec<PathBuf>>, quarantine_dir: &Path) {
    // Create quarantine directory if it doesn't exist
    if !quarantine_dir.exists() {
        let _ = fs::create_dir_all(quarantine_dir);
    }

    for group in duplicate_groups {
        // Keep the first file, quarantine the rest
        for file in group.iter().skip(1) {
            if let Some(filename) = file.file_name() {
                let dest_path = quarantine_dir.join(filename);

                // Try to move the file to quarantine
                if let Err(e) = fs::rename(file, &dest_path) {
                    eprintln!("Failed to move {:?} to quarantine: {}", file, e);
                }
            }
        }
    }
}
