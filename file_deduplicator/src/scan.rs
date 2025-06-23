use walkdir::WalkDir;
use std::path::PathBuf;

// Collect all regular files using WalkDir
pub fn collect_files(folder: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();

    for entry in WalkDir::new(folder) {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.is_file() {
                files.push(path.to_path_buf());
            }
        }
    }

    files
}
