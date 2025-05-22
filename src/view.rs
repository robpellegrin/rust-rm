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
use std::env;
use std::fs;

/// TODO --> Display relevant metadata in a table format.

pub fn list_trash_contents() {
    // Get the path to the trash directory
    let home_dir = env::home_dir().expect("Failed to get home directory");
    let trash_dir = home_dir.join("trash");

    // Check if the trash directory exists
    if !trash_dir.exists() {
        println!("Trash directory does not exist at {:?}", trash_dir);
        return;
    }

    // List all files and directories in the trash directory
    match fs::read_dir(trash_dir) {
        Ok(entries) => {
            let mut has_files = false;
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let entry_name = entry.file_name();
                        println!("{}", entry_name.to_string_lossy());
                        has_files = true;
                    }
                    Err(e) => {
                        println!("Failed to read entry: {}", e);
                    }
                }
            }

            if !has_files {
                println!("The trash is empty.");
            }
        }
        Err(e) => {
            println!("Failed to read the trash directory: {}", e);
        }
    }
}
