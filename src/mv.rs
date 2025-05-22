use std::fs;
use std::path::{Path, PathBuf};

use crate::utils;

pub fn move_to_trash(source: &str) -> std::io::Result<()> {
    // Expand the ~ in the destination path
    let trash_dir = utils::expand_tilde("~/trash");

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
