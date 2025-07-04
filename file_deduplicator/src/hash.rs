// src/hash.rs
use std::fs::File;
use std::io::{Read, Result};
use std::path::{Path, PathBuf};

use sha2::{Sha256, Digest};
use rayon::prelude::*;

// Hash a file using SHA-256 and return the hash as a hex string
pub fn hash_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 1024];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(format!("{:x}", hash))
}

// Hash multiple files in parallel using Rayon
pub fn hash_files_parallel(files: Vec<PathBuf>) -> Vec<(PathBuf, String)> {
    files
        .par_iter()
        .filter_map(|path| {
            if let Ok(hash) = hash_file(path) {
                Some((path.clone(), hash))
            } else {
                None
            }
        })
        .collect()
}
