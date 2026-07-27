use rpassword::prompt_password;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use zeroize::Zeroize;

/// A wrapper string that automatically overwrites its memory with zeroes when dropped.
#[derive(Zeroize)]
#[zeroize(drop)]
pub struct SecretString(String);

impl SecretString {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub fn get_password_if_encrypted(config_path: &Path) -> io::Result<Option<SecretString>> {
    let mut file = File::open(config_path)?;
    // Read up to the first 256 bytes to safely bypass any BOMs or leading whitespace
    let mut buffer = [0u8; 256];

    let bytes_read = file.read(&mut buffer)?;
    let header_chunk = String::from_utf8_lossy(&buffer[..bytes_read]);

    // Check if the chunk contains the encryption signature anywhere
    if header_chunk.contains("RCLONE_ENCRYPT_V0:") {
        println!("The rclone config file is encrypted.");
        let pw = prompt_password("Enter rclone config password: ")?;
        Ok(Some(SecretString::new(pw)))
    } else {
        Ok(None)
    }
}