use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "rclone_scrambler",
    about = "Securely scrambles file modification dates on an rclone remote."
)]
pub struct Args {
    /// Path to the rclone configuration file
    #[arg(short, long)]
    pub config: PathBuf,

    /// Number of concurrent threads to use
    #[arg(short, long, default_value_t = 16)]
    pub threads: usize,

    /// Enable verbose logging of modified files and their new timestamps
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}