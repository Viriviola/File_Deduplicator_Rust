
use std::fs::{self, File}; //file creation and reading
use std::io::Write; //write test files
use std::path::Path;  //for file paths

use file_deduplicator::{scan, filter, hash, group, quarantine};
//use file_deduplicator::{model::*, report::*};


use tempfile::tempdir; //for temporary directories cleanup

//Helper function to create test files
fn create_test_file(dir: &Path, name: &str, content: &str) {
    let file_path = dir.join(name);
    let mut file = File::create(&file_path).expect("Failed to create test file");
    write!(file, "{}", content).expect("Failed to write test file");
}

// extension based filtering test
#[test]
fn test_scan_and_filter() {
    let dir = tempdir().unwrap();
    create_test_file(dir.path(), "file1.txt", "hello");
    create_test_file(dir.path(), "file2.txt", "hello");
    create_test_file(dir.path(), "file3.log", "hello");

    let files = scan::collect_files(dir.path().to_str().unwrap());
    assert_eq!(files.len(), 3);  // 3 files created

    let filtered = filter::filter_by_extension(files, &["txt"]);
    assert_eq!(filtered.len(), 2);  //2 files with .txt extension
}

//parallel hashing, grouping of identical content files
#[test]
fn test_hash_and_grouping() {
    let dir = tempdir().unwrap();
    create_test_file(dir.path(), "a.txt", "same");
    create_test_file(dir.path(), "b.txt", "same");
    create_test_file(dir.path(), "c.txt", "different");

    let files = scan::collect_files(dir.path().to_str().unwrap());
    let hashed = hash::hash_files_parallel(files);
    let grouped = group::group_by_hash(hashed);
    let duplicates = group::find_duplicates(grouped);

    assert_eq!(duplicates.len(), 1); // 1 group of duplicates
    assert_eq!(duplicates[0].len(), 2); // 2 files with the same content
}

//one file should be quarantined other left untouched
#[test]
fn test_quarantine() {
    let dir = tempdir().unwrap();
    create_test_file(dir.path(), "x.txt", "copy");
    create_test_file(dir.path(), "y.txt", "copy");

    let files = scan::collect_files(dir.path().to_str().unwrap());
    let hashed = hash::hash_files_parallel(files);
    let grouped = group::group_by_hash(hashed);
    let duplicates = group::find_duplicates(grouped);

    let quarantine_dir = dir.path().join("quarantine");
    quarantine::quarantine_duplicates(duplicates.clone(), &quarantine_dir);

    // Only one file should be quarantined
    assert!(quarantine_dir.exists());
    let count = fs::read_dir(&quarantine_dir).unwrap().count();
    assert_eq!(count, 1); // 1 file quarantined
}

#[test]
fn test_report_files_created() {
    use file_deduplicator::{model::*, report::*};

    let dir = tempdir().unwrap();
    let json_path = dir.path().join("test_report.json");
    let html_path = dir.path().join("test_report.html");

    // Create a report to test file creation
    // mimics a report: 2 scanned files, 1 duplicate group
    //hardcoded data for testing

    let report = Report {
        scanned: 2,
        duplicates_found: 1,
        duplicate_groups: vec![DuplicateGroup {
            hash: "dummyhash".to_string(),
            files: vec!["a.txt".to_string(), "b.txt".to_string()],
        }],
        space_savings: 1024,
        space_savings_size: "1.00 KB".to_string(),
    };

    // Write the report to JSON and HTML files
    assert!(write_json_report(&report, &json_path).is_ok());
    assert!(write_html_report(&report, &html_path).is_ok());

    // confirmation of file creation
    assert!(json_path.exists());
    assert!(html_path.exists());
} 
// tempdir has automatically cleaned up after the test when it goes out of scope
