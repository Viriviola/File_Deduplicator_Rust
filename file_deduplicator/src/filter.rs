// src/filter.rs
use std::path::PathBuf;
use std::fs;
use regex::Regex;
use chrono::{DateTime, Utc};

// Filter files by minimum size (in bytes)
pub fn filter_by_size(files: Vec<PathBuf>, min_size: u64) -> Vec<PathBuf> {
    let mut filtered = Vec::new();

    for file in files {
        if let Ok(metadata) = fs::metadata(&file) {
            if metadata.len() >= min_size {
                filtered.push(file);
            }
        }
    }

    filtered
}

// Filter files by extension (e.g., ["jpg", "txt"])
pub fn filter_by_extension(files: Vec<PathBuf>, extensions: &[&str]) -> Vec<PathBuf> {
    files.into_iter()
        .filter(|file| {
            if let Some(ext) = file.extension() {
                if let Some(ext_str) = ext.to_str() {
                    return extensions.contains(&ext_str);
                }
            }
            false
        })
        .collect()
}

// Filter files by last modified date (keep files modified after given time)
pub fn filter_by_modified_date(files: Vec<PathBuf>, after: DateTime<Utc>) -> Vec<PathBuf> {
    let mut filtered = Vec::new();

    for file in files {
        if let Ok(metadata) = fs::metadata(&file) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(modified_dt) = modified.duration_since(std::time::UNIX_EPOCH) {
                    let file_time = DateTime::<Utc>::from(std::time::UNIX_EPOCH + modified_dt);
                    if file_time > after {
                        filtered.push(file);
                    }
                }
            }
        }
    }

    filtered
}

// Filter files by filename pattern using regex
pub fn filter_by_name_pattern(files: Vec<PathBuf>, pattern: &str) -> Vec<PathBuf> {
    let regex = Regex::new(pattern);
    if let Ok(regex) = regex {
        files.into_iter()
            .filter(|file| {
                if let Some(name) = file.file_name() {
                    if let Some(name_str) = name.to_str() {
                        return regex.is_match(name_str);
                    }
                }
                false
            })
            .collect()
    } else {
        Vec::new() // return empty if regex is invalid
    }
}
