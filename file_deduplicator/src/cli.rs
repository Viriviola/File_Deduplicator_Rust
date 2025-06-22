//For taking CLI input from the user
use clap::Parser;

// holds arguments given by the user
#[derive(Parser, Debug)]
#[command(name = "File Deduplicator")]

pub struct Cli {
    // folder path 
    pub path: String,

    // report type: json or html
    #[arg(long, default_value = "json")]
    pub report: String,

    // move duplicates to a quarantine folder (default: true)
    #[arg(long, default_value_t = true)]
    pub quarantine: bool,
}

// reads and parses the command line input
pub fn get_args() -> Cli {
    Cli::parse()
}
