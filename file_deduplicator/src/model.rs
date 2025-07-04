// src/models.rs
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

// Represents a single file and its hash
#[derive(Serialize, Deserialize, Debug)]
pub struct FileHash {
    pub path: PathBuf,
    pub hash: String,
}

// Represents a group of duplicate files
#[derive(Serialize, Deserialize, Debug)]
// pub struct DuplicateGroup {
//     pub hash: String,
//     pub files: Vec<PathBuf>,
// }
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<String>, // change from PathBuf to String
}


// Report structure for JSON or HTML output
#[derive(Serialize, Deserialize, Debug)]
pub struct Report {
    pub scanned: usize,
    pub duplicates_found: usize,
    pub duplicate_groups: Vec<DuplicateGroup>,
    pub space_savings: u64,
    pub space_savings_size: String, // 👈 new field
}
