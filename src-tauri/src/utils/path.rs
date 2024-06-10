//! Copyright: 2024 Lizc. All rights reserved.
//! License: MIT License
//! You may obtain a copy of the License at https://opensource.org/licenses/MIT
//!
//! Author: Lizc
//! Created Data: 2024-06-05
//!
//! Description: Path utils.

use std::fs::OpenOptions;

pub fn ensure_file_exists(file: &str) -> std::io::Result<()> {
    let path = std::path::Path::new(file);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let _ = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(file)?;
    Ok(())
}
