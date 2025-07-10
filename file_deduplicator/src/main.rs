mod cli;
mod scan;
mod filter;
mod hash;
mod group;
mod quarantine;
mod model;
mod report;


use std::path::Path;
use model::{DuplicateGroup, Report};
use report::{write_json_report, write_html_report};
//use chrono::Utc;

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    match bytes {
        b if b >= GB => format!("{:.2} GB", b as f64 / GB as f64),
        b if b >= MB => format!("{:.2} MB", b as f64 / MB as f64),
        b if b >= KB => format!("{:.2} KB", b as f64 / KB as f64),
        b => format!("{} bytes", b),
    }
}

fn main() {
    // step 1: Getting CLI arguments
    let args = cli::get_args();
    println!("Scanning folder: {}", args.path);

    // Step 2: Scaning files from user-provided folder
    let files = scan::collect_files(&args.path);
    println!("Files found: {}", files.len());

    // Step 3: Applying filters
    //size filter
    let filtered = filter::filter_by_size(files, 1);
    println!("After size filter: {}", filtered.len());
    
    //extension filter
    let filtered = filter::filter_by_extension(filtered, &["txt"]);
    println!("After extension filter: {}", filtered.len());
    
    //date filter
    // let filtered = filter::filter_by_modified_date(filtered, Utc::now() - chrono::Duration::days(30));
    
    //name pattern filter
    let filtered = filter::filter_by_name_pattern(filtered, ".*");
    println!("After name pattern filter: {}", filtered.len());

    // Step 4: Hashing
    let hashed = hash::hash_files_parallel(filtered);
    println!("Files hashed: {}", hashed.len());
    let scanned = hashed.len();

    // Step 5: Grouping
    let groups = group::group_by_hash(hashed.clone());
    let duplicates = group::find_duplicates(groups);
    println!("Duplicate groups found: {}", duplicates.len());


    // Step 6: Calculate space savings before quarantine
    let space_savings = duplicates.iter().map(|group| {
        group.iter().skip(1).map(|file| {
            std::fs::metadata(file).map(|m| m.len()).unwrap_or(0)
        }).sum::<u64>()
    }).sum();
    let space_savings_size = format_bytes(space_savings);

    // Step 7: Quarantine (enabled by default)
    if args.quarantine {
        let quarantine_dir = Path::new("quarantine");
        quarantine::quarantine_duplicates(duplicates.clone(), quarantine_dir);
    }

    // Step 8: Generating report
    let report = Report {
        scanned,
        duplicates_found: duplicates.len(),
        duplicate_groups: duplicates.into_iter().map(|g| DuplicateGroup {
            hash: "duplicate_hash".to_string(),
            files: g.iter().map(|p| p.display().to_string()).collect(),
        }).collect(),
        space_savings,
        space_savings_size,
    };

    if args.report == "json" {
        let _ = write_json_report(&report, Path::new("report.json"));
    } else if args.report == "html" {
        let _ = write_html_report(&report, Path::new("report.html"));
    }

    println!("Done. Report generated.");
}
