mod cli;
mod crypto;
mod rclone;
mod scrambler;

use clap::Parser;
use dialoguer::{Input, Select};
use std::process::exit;

fn main() {
    let args = cli::Args::parse();

    // 1. Determine if the config is encrypted and prompt for password securely
    let password = match crypto::get_password_if_encrypted(&args.config) {
        Ok(Some(pw)) => Some(pw),
        Ok(None) => None,
        Err(e) => {
            eprintln!("Error reading config file: {}", e);
            exit(1);
        }
    };

    let rclone_runner = rclone::RcloneRunner::new(args.config.clone(), password);

    // 2. Fetch and choose the remote
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

    // 3. Ask for the directory path
    let directory: String = Input::new()
        .with_prompt(format!("Enter the directory path on '{}' (leave blank for root)", selected_remote))
        .allow_empty(true)
        .interact_text()
        .unwrap();

    // 4. Scramble the files
    println!("Fetching files list. This might take a moment...");
    if let Err(e) = scrambler::run_scrambler(&rclone_runner, selected_remote, &directory, args.verbose) {
        eprintln!("Error during scrambling process: {}", e);
        exit(1);
    }

    println!("\nOperation completed successfully.");
}