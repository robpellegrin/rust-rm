/// =====================================================================
/// Project Name: rust rm
/// Description: An enhanced version of the common rm utility.
/// Author: Robert Pellegrin
/// Date: 2025-05-17
/// Version: 0.0.1
/// License: MIT
/// Repository:
/// =====================================================================
///
use std::path::PathBuf;

pub fn expand_tilde(path: &str) -> PathBuf {
    let home = dirs::home_dir().expect("Could not find home directory");

    let path = if path.starts_with("~") {
        // Remove ~ and join with home dir
        home.join(&path[2..])
    } else {
        // Return the path as is
        PathBuf::from(path)
    };

    return path;
}
