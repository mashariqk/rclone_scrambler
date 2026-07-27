use crate::crypto::SecretString;
use std::io;
use std::path::PathBuf;
use std::process::{Command, Output, Child, Stdio};

pub struct RcloneRunner {
    config_path: PathBuf,
    password: Option<SecretString>,
}

impl RcloneRunner {
    pub fn new(config_path: PathBuf, password: Option<SecretString>) -> Self {
        Self {
            config_path,
            password,
        }
    }

    fn base_cmd(&self) -> Command {
        let mut cmd = Command::new("rclone");
        cmd.arg("--config").arg(&self.config_path);

        if let Some(pw) = &self.password {
            cmd.env("RCLONE_CONFIG_PASS", pw.as_str());
        }
        cmd
    }

    fn handle_output(output: io::Result<Output>) -> Result<String, String> {
        match output {
            Ok(out) if out.status.success() => {
                Ok(String::from_utf8_lossy(&out.stdout).to_string())
            }
            Ok(out) => Err(String::from_utf8_lossy(&out.stderr).to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn list_remotes(&self) -> Result<Vec<String>, String> {
        let output = self.base_cmd().arg("listremotes").output();

        let stdout = Self::handle_output(output)?;
        let remotes: Vec<String> = stdout
            .lines()
            .map(|line| line.trim_end_matches(':').to_string())
            .filter(|s| !s.is_empty())
            .collect();

        Ok(remotes)
    }

    /// Spawns an `rclone lsf` process and returns a reader to stream its output
    pub fn stream_files(&self, remote: &str, dir: &str) -> Result<(Child, io::BufReader<std::process::ChildStdout>), String> {
        let target = if dir.is_empty() {
            format!("{}:", remote)
        } else {
            format!("{}:{}", remote, dir)
        };

        // We use 'lsf' to get raw paths line-by-line instead of loading massive JSON arrays
        let mut child = self
            .base_cmd()
            .arg("lsf")
            .arg(&target)
            .arg("-R") // Recursive
            .arg("--files-only")
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn rclone lsf: {}", e))?;

        let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let reader = io::BufReader::new(stdout);

        Ok((child, reader))
    }

    pub fn touch_file(&self, remote: &str, dir: &str, file_path: &str, timestamp: &str) -> Result<(), String> {
        let target = if dir.is_empty() {
            format!("{}:{}", remote, file_path)
        } else {
            format!("{}:{}/{}", remote, dir, file_path)
        };

        let output = self
            .base_cmd()
            .arg("touch")
            .arg(&target)
            .arg("--timestamp")
            .arg(timestamp)
            .output();

        Self::handle_output(output).map(|_| ())
    }
}