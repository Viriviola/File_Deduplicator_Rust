use std::fs::File;
use std::io::{Read, Result};
use std::path::{Path, PathBuf};

use sha2::{Sha256, Digest};
use rayon::prelude::*;

// Hash a file using SHA-256 and return the hash as a hex string
pub fn hash_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?; //opeming the file
    let mut hasher = Sha256::new(); // Creating a new SHA-256 hasher
    let mut buffer = [0u8; 1024];  //reads the file in chunks of 1024 bytes

    loop {
        let bytes_read = file.read(&mut buffer)?; // Reading the file into the buffer
        if bytes_read == 0 {
            break; //end of file
        }
        hasher.update(&buffer[..bytes_read]); // Updating the hasher with the bytes read
    }

    let hash = hasher.finalize();
    Ok(format!("{:x}", hash)) //returns the hash as a hex string
}

// Hash multiple files in parallel using Rayon
pub fn hash_files_parallel(files: Vec<PathBuf>) -> Vec<(PathBuf, String)> {
    files
        .par_iter() //Rayon parallel iterator 
        .filter_map(|path| {
            if let Ok(hash) = hash_file(path) {
                Some((path.clone(), hash))
            } else {
                None //skip files that cannot be hashed or read
            }
        })
        .collect() // Collects the results into a vector of tuples (PathBuf, String)
}
