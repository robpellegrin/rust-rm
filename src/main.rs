#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

mod restore;

/// =====================================================================
/// Project Name: rust rm
/// Description: An enhanced version of the common rm command.
/// Author: Robert Pellegrin
/// Date: 2025-05-17
/// Version: 0.0.1
/// License: MIT
/// Repository:
/// =====================================================================
///
/// # Overview
/// This project enhances the traditional `rm` command by adding a trash bin
/// feature. Files deleted with the `rm` command are instead moved to a trash
/// bin directory, making them recoverable if deleted by accident. The tool
/// also integrates with GUI-based trash bins (e.g., Dolphin) so that users
/// can restore files using their desktop environment's trash interface.
///
/// ## Key Features:
/// - Trash bin for `rm`-deleted files, allowing recovery. This is currently located
/// at ~/trash
/// - Command-line interface mimics the behavior of the traditional `rm` command.
///
/// ## TODO
/// - Store metadata along with deleted files to make restoring easier.
/// - Restore files from trash using CLI
/// - View trash contents via CLI.
/// - Config file to allow user to specify custom path to trash, size limitations, etc.
/// - Integration with GUI trash bins (e.g., Dolphin, Nautilus) for restoration.

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("Missing args");
    }

    // Loop over each argument (excluding the first one, which is the program name)
    for arg in args.iter().skip(1) {
        if let Err(e) = mv(arg) {
            eprintln!("Error moving file '{}' to trash: {}", arg, e);
            std::process::exit(-1);
        }
    }
}

fn expand_tilde(path: &str) -> PathBuf {
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

fn mv(source: &str) -> std::io::Result<()> {
    // Expand the ~ in the destination path
    let trash_dir = expand_tilde("~/trash");

    // Get the filename from the source path
    let source_path = Path::new(source);

    let filename = match source_path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(), // Convert OsStr to String
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Source path does not contain a valid filename",
            ));
        }
    };

    // Join the trash directory with the filename
    let trash_path = trash_dir.join(filename);

    // Try to rename (move) the file to the trash directory
    fs::rename(source, &trash_path)?;

    Ok(())
}
