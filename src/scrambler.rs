use crate::rclone::RcloneRunner;
use chrono::{TimeZone, Utc};
use rand::RngExt;
use rayon::prelude::*;
use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub fn run_scrambler(
    runner: &RcloneRunner,
    remote: &str,
    directory: &str,
    verbose: bool,
) -> Result<(), String> {
    let (mut child, reader) = runner.stream_files(remote, directory)?;

    let count = AtomicUsize::new(0);
    let output_lock = Mutex::new(()); // Prevents terminal text overlapping

    // Process the streamed lines in parallel
    reader
        .lines()
        .filter_map(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .par_bridge() // <--- This hands off incoming lines to the thread pool instantly
        .for_each(|file_path| {
            // Generate random timestamp inside the thread
            let mut rng = rand::rng();
            let random_ts = rng.random_range(0..2_524_608_000);
            let dt = Utc.timestamp_opt(random_ts, 0).unwrap();
            let formatted_time = dt.format("%Y-%m-%dT%H:%M:%S").to_string();

            if let Err(e) = runner.touch_file(remote, directory, &file_path, &formatted_time) {
                let _lock = output_lock.lock().unwrap();
                eprintln!("\nFailed to process '{}': {}", file_path, e);
                return;
            }

            let current_count = count.fetch_add(1, Ordering::Relaxed) + 1;

            // Secure terminal output so text doesn't tear
            let _lock = output_lock.lock().unwrap();
            if verbose {
                println!("Modified: '{}' -> {}", file_path, formatted_time);
            } else {
                print!("\rFiles modified: {}", current_count);
                let _ = io::stdout().flush();
            }
        });

    let status = child.wait().map_err(|e| format!("Failed to wait on child: {}", e))?;
    let final_count = count.load(Ordering::Relaxed);

    if !verbose && final_count > 0 {
        println!();
    }

    if final_count == 0 {
        println!("No files found, or directory does not exist.");
    } else if !status.success() {
        eprintln!("Warning: The rclone list process exited with status: {}", status);
    }

    Ok(())
}