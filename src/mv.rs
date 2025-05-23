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
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::{fs, io};

/// Moves a file (or directory) to the trash.
pub fn move_to_trash(source: &str, allow_dir_removal: bool) -> std::io::Result<()> {
    let trash_dir = dirs::home_dir()
        .map(|home| home.join("trash"))
        .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Could not determine home directory"))?;

    // Get the filename from the source path
    let source_path = Path::new(source);

    // Ensure the directory exists
    fs::create_dir_all(&trash_dir)?; // No error if it already exists

    // Check if the source path is a directory
    if source_path.is_dir() && !allow_dir_removal {
        return Err(io::Error::new(
            ErrorKind::InvalidInput,
            "Cannot move a directory to trash",
        ));
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

    // Get a resolved trash path that avoids naming conflicts
    let trash_path = resolve_naming_conflict(&trash_dir, &filename);

    // Try to rename (move) the file to the trash directory
    fs::rename(source, &trash_path)?;

    Ok(())
}

/// Given a target directory and desired filename, append suffixes like (1), (2), etc. if needed
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
