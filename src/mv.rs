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
use chrono::Local;
use std::{
    fs::{self, File},
    io::{self, ErrorKind, Write},
    path::{Path, PathBuf},
};

use crate::args::Args;

/// Moves a file (or directory) to the trash.
pub fn move_to_trash(source: &str, args: &Args) -> std::io::Result<()> {
    let trash_dir_files = dirs_next::home_dir()
        .map(|home| home.join(".local/share/Trash/files"))
        .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Could not determine home directory"))?;

    // Get the filename from the source path
    let source_path = Path::new(source);

    // TODO --> If the directory does not exist, silently create it.
    // Ensure the directory exists
    fs::create_dir_all(&trash_dir_files)?; // No error if it already exists

    // Check if the file is a symlink before proceeding. If it is, delete it instead of attempting
    // to move it to the trash.
    if fs::symlink_metadata(source)?.file_type().is_symlink() {
        fs::remove_file(source)?;
        return Ok(());
    }

    // Check if the source path is a directory
    if source_path.is_dir() && !args.recursive {
        return Err(io::Error::new(ErrorKind::InvalidInput, "Is a directory"));
    }

    let filename = match source_path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(), // Convert OsStr to String
        None => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Source path does not contain a valid filename",
            ));
        }
    };

    if args.skip_trash {
        if source_path.is_dir() {
            fs::remove_dir_all(source)?;
        } else {
            fs::remove_file(source)?;
        }
        return Ok(());
    }

    // Get a resolved trash path that avoids naming conflicts
    let trash_path = resolve_naming_conflict(&trash_dir_files, &filename);

    match fs::canonicalize(source_path) {
        Ok(abs_path) => {
            if let Some(path_str) = abs_path.to_str() {
                if let Err(e) = create_metadata_file(path_str) {
                    eprintln!("Error creating metadata file for {}: {}", source, e);
                    return Ok(());
                }
            } else {
                eprintln!("Path contains invalid UTF-8: {}", abs_path.display());
                return Ok(());
            }
        }
        Err(e) => {
            eprintln!("Error resolving path for {}: {}", source, e);
            return Ok(());
        }
    }

    if args.interactive {
        let prompt = format!("move '{}' to the trash?", source);
        if !confirm(&prompt)? {
            return Ok(());
        }
    }

    if args.verbose {
        println!("removed '{}'", source);
    }

    // Try to rename (move) the file to the trash directory
    fs::rename(source, &trash_path)?;

    if args.verbose {
        println!("removed '{}'", source);
    }

    Ok(())
}

/// Create a text file with metadata about the file being sent to the trash.
/// Metadata includes the original path of the file, as well as the time and
/// date it was moved to the trash.
fn create_metadata_file(filename: &str) -> io::Result<()> {
    let current_date_time = Local::now();
    let formatted_date_time = current_date_time.format("%Y-%m-%dT%H:%M:%S").to_string();

    let trash_dir_metadata = dirs_next::home_dir()
        .map(|home| home.join(".local/share/Trash/info"))
        .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Could not determine home directory"))?;

    fs::create_dir_all(&trash_dir_metadata)?; // Ensure the directory exists

    let new_filename = Path::new(filename)
        .file_name()
        .and_then(|s| s.to_str())
        .map(|s| format!("{}.trashinfo", s))
        .ok_or_else(|| io::Error::new(ErrorKind::InvalidInput, "Invalid UTF-8 in filename"))?;

    let final_filename = resolve_naming_conflict(&trash_dir_metadata, &new_filename);
    let full_path = trash_dir_metadata.join(&final_filename);

    let mut file = File::create(&full_path)?;
    writeln!(file, "[Trash Info]")?;
    writeln!(file, "Path={}", filename)?;
    writeln!(file, "DeletionDate={}", formatted_date_time)?;

    // File is already in place, no need to move/rename
    Ok(())
}

/// Given a target directory and desired filename, append suffixes like (1), (2), etc if needed
/// in order to resolve naming conflicts.
fn resolve_naming_conflict(trash_dir: &Path, filename: &str) -> PathBuf {
    let mut candidate = trash_dir.join(filename);
    let mut counter = 1;

    while candidate.exists() {
        let path = Path::new(filename);
        let new_filename = format!(
            "{}({}){}",
            path.file_stem().unwrap_or_default().to_string_lossy(),
            counter,
            path.extension()
                .map_or(String::new(), |ext| format!(".{}", ext.to_string_lossy()))
        );
        candidate = trash_dir.join(new_filename);
        counter += 1;
    }

    candidate
}

/// Prompts the user for a yes/no confirmation.
/// Returns true if the user enters 'y' or 'Y'.
fn confirm(prompt: &str) -> io::Result<bool> {
    print!("{} [y/N]: ", prompt);

    // Make sure prompt shows up before input
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let response = input.trim().to_lowercase();

    Ok(response == "y")
}
