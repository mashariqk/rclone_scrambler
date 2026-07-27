use crate::crypto::SecretString;
use serde::Deserialize;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Deserialize)]
pub struct RcloneFile {
    #[serde(rename = "Path")]
    pub path: String,
}

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

    /// Builds a base command with the config file and password environment set
    fn base_cmd(&self) -> Command {
        let mut cmd = Command::new("rclone");
        cmd.arg("--config").arg(&self.config_path);

        if let Some(pw) = &self.password {
            // This is only exposed to the specific child process, not globally
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

    pub fn list_files(&self, remote: &str, dir: &str) -> Result<Vec<RcloneFile>, String> {
        let target = if dir.is_empty() {
            format!("{}:", remote)
        } else {
            format!("{}:{}", remote, dir)
        };

        let output = self
            .base_cmd()
            .arg("lsjson")
            .arg(&target)
            .arg("-R") // Recursive
            .arg("--files-only")
            .output();

        let stdout = Self::handle_output(output)?;
        serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse JSON: {}", e))
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
use std::io;