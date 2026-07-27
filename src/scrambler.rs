use crate::rclone::RcloneRunner;
use chrono::{TimeZone, Utc};
use rand::RngExt; // <-- Updated trait import for rand 0.10+
use std::io::{self, Write};

pub fn run_scrambler(
    runner: &RcloneRunner,
    remote: &str,
    directory: &str,
    verbose: bool,
) -> Result<(), String> {
    let files = runner.list_files(remote, directory)?;

    if files.is_empty() {
        println!("No files found in the specified directory.");
        return Ok(());
    }

    let mut count = 0;
    let total = files.len();

    let mut rng = rand::rng();

    for file in files {
        // random_range is provided by the RngExt trait in rand 0.10+
        let random_ts = rng.random_range(0..2_524_608_000);

        let dt = Utc.timestamp_opt(random_ts, 0).unwrap();
        // rclone touch format expects: YYYY-MM-DDTHH:MM:SS
        let formatted_time = dt.format("%Y-%m-%dT%H:%M:%S").to_string();

        if let Err(e) = runner.touch_file(remote, directory, &file.path, &formatted_time) {
            eprintln!("\nFailed to process '{}': {}", file.path, e);
            continue;
        }

        count += 1;

        if verbose {
            println!("Modified: '{}' -> {}", file.path, formatted_time);
        } else {
            // Overwrite the current line with the updated count
            print!("\rFiles modified: {} / {}", count, total);
            io::stdout().flush().unwrap();
        }
    }

    Ok(())
}