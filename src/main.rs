mod cli;
mod crypto;
mod rclone;
mod scrambler;

use clap::Parser;
use dialoguer::{Input, Select};
use std::process::exit;

fn main() {
    let args = cli::Args::parse();

    // Initialize the thread pool for parallel processing
    if let Err(e) = rayon::ThreadPoolBuilder::new().num_threads(args.threads).build_global() {
        eprintln!("Failed to initialize thread pool: {}", e);
        exit(1);
    }

    let password = match crypto::get_password_if_encrypted(&args.config) {
        Ok(Some(pw)) => Some(pw),
        Ok(None) => None,
        Err(e) => {
            eprintln!("Error reading config file: {}", e);
            exit(1);
        }
    };

    let rclone_runner = rclone::RcloneRunner::new(args.config.clone(), password);

    println!("Fetching remotes...");
    let remotes = match rclone_runner.list_remotes() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to list remotes (is your password correct?): {}", e);
            exit(1);
        }
    };

    if remotes.is_empty() {
        println!("No remotes found in the configuration file.");
        exit(0);
    }

    let selection = Select::new()
        .with_prompt("Choose the remote to work on")
        .items(&remotes)
        .default(0)
        .interact()
        .unwrap();

    let selected_remote = &remotes[selection];

    let directory: String = Input::new()
        .with_prompt(format!("Enter the directory path on '{}' (leave blank for root)", selected_remote))
        .allow_empty(true)
        .interact_text()
        .unwrap();

    let target_path = if directory.is_empty() {
        format!("{}:", selected_remote)
    } else {
        format!("{}:{}", selected_remote, directory)
    };

    println!("Fetching files from '{}' using {} threads. This might take a moment...", target_path, args.threads);
    if let Err(e) = scrambler::run_scrambler(&rclone_runner, selected_remote, &directory, args.verbose) {
        eprintln!("Error during scrambling process: {}", e);
        exit(1);
    }

    println!("\nOperation completed successfully.");
}