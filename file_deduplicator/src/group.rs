use std::collections::HashMap;
use std::path::PathBuf;

// Group files by their hash value
pub fn group_by_hash(file_hashes: Vec<(PathBuf, String)>) -> HashMap<String, Vec<PathBuf>> {
    let mut groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for (path, hash) in file_hashes {
        groups.entry(hash).or_insert_with(Vec::new).push(path);
    }

    groups
}

// Return only groups with duplicates (more than 1 file per hash)
pub fn find_duplicates(groups: HashMap<String, Vec<PathBuf>>) -> Vec<Vec<PathBuf>> {
    groups
        .into_iter()
        .filter_map(|(_, paths)| {
            if paths.len() > 1 {
                Some(paths)
            } else {
                None
            }
        })
        .collect()
}
