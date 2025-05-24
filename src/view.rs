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
use tabled::settings::style::Style;
use tabled::{Table, Tabled};

#[allow(unused)]
// This function will likely be removed in favor of the function below that
// prints out the trash contents in a table.
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

#[derive(Tabled)]
struct TrashEntry {
    file: String,
    path: String,
    date: String,
}

pub fn list_trash_contents_table() {
    let home_dir = env::home_dir().expect("Failed to get home directory");
    let trash_dir = home_dir.join("trash");

    if !trash_dir.exists() {
        println!("Trash directory does not exist at {:?}", trash_dir);
        return;
    }

    let mut entries: Vec<TrashEntry> = Vec::new();

    match fs::read_dir(&trash_dir) {
        Ok(dir_entries) => {
            let mut has_files = false;

            for entry in dir_entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    entries.push(TrashEntry {
                        file: file_name,
                        path: String::new(), // Placeholder
                        date: String::new(),  // Placeholder
                    });
                    has_files = true;
                }
            }

            if has_files {
                let mut table = Table::new(&entries);
                table.with(Style::sharp());
                println!("{}", table)
            } else {
                println!("The trash is empty.");
            }
        }

        Err(e) => {
            eprintln!("Failed to read the trash directory: {}", e);
        }
    }
}
