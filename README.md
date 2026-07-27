# 🎲 rclone_scrambler

A secure, high-performance Rust CLI utility that recursively randomizes file modification timestamps (`mtime`) on `rclone` remotes.

Whether you are obfuscating metadata for privacy or testing timestamp-based synchronization behaviors, `rclone_scrambler` provides a clean, menu-driven interface to safely and rapidly scramble your cloud files.

## ✨ Features

- ⚡ **High Performance & Multi-threaded:** Uses thread-pooling to process multiple files concurrently, drastically speeding up operations on remotes with high latency or thousands of files.
- 🌊 **Memory-Efficient Streaming:** Streams file paths directly from the remote rather than loading massive JSON arrays into memory. It starts scrambling the moment the first file is found, regardless of directory size.
- 🔒 **Secure Password Handling:** Automatically detects encrypted `rclone` configurations. It prompts for your password securely in the terminal and uses the `zeroize` crate to wipe it from memory the moment it is no longer needed.
- 🕵️ **No Environment Leaks:** The configuration password is never logged, saved to disk, or exported to your global shell environment. It is passed strictly to the `rclone` child process.
- 🖥️ **Interactive Interface:** Provides a slick, menu-driven UI to select your target remote and input directory paths.
- 📁 **Recursive Processing:** Traverses the target directory and scrambles the modification time of every file inside to a random date between 1970 and 2050.

## 🛠️ Prerequisites

Before you begin, ensure you have the following installed:
- **[Rust & Cargo](https://rustup.rs/)** (Edition 2021 / Rust 1.70+)
- **[rclone](https://rclone.org/)** (Must be installed and accessible in your system's `PATH`)

## 🚀 Installation

Clone the repository and compile the project using Cargo:

```bash
git clone git@github.com:mashariqk/rclone_scrambler.git
cd rclone_scrambler
cargo build --release