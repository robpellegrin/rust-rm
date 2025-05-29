///
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
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tabled::settings::style::Style;
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct TrashEntry {
    file: String,
    path: String,
    date: String,
}

pub fn list_trash_contents_table() {
    let home_dir = dirs_next::home_dir().expect("Failed to get home directory");
    let trash_dir = home_dir.join(".local/share/Trash/files");

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
                    let original_path = get_info_from_trashinfo(&file_name, "Path=")
                        .unwrap_or_else(|| "Unknown".to_string());
                    let date_info = get_info_from_trashinfo(&file_name, "DeletionDate=")
                        .unwrap_or_else(|| "Unknown".to_string());

                    entries.push(TrashEntry {
                        file: file_name,
                        path: original_path,
                        date: date_info,
                    });
                    has_files = true;
                }
            }

            if has_files {
                println!("{}", Table::new(entries).with(Style::sharp()).to_string());
            } else {
                println!("The trash is empty.");
            }
        }

        Err(e) => {
            eprintln!("Failed to read the trash directory: {}", e);
        }
    }
}

/// Extracts info from .trashinfo file. i.e. path or datetime.
fn get_info_from_trashinfo(file_name: &str, search_term: &str) -> Option<String> {
    let home_dir = dirs_next::home_dir()?;
    let info_path = home_dir
        .join(".local/share/Trash/info")
        .join(format!("{}.trashinfo", file_name));

    if !info_path.exists() {
        return None;
    }

    let file = File::open(info_path).ok()?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if line.starts_with(search_term) {
                return Some(line[search_term.len()..].trim().to_string());
            }
        }
    }

    None
}
