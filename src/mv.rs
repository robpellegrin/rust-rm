use std::fs;
use std::path::Path;

use crate::utils;

// TODO --> Manage name conflicts for existing files in trash.

pub fn mv(source: &str) -> std::io::Result<()> {
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

    // Generate the initial trash path
    let mut trash_path = trash_dir.join(&filename);

    // Check if the file exists in the trash directory
    let mut counter = 1;
    while trash_path.exists() {
        // If a conflict, append a suffix like (1), (2), etc.
        let path = Path::new(&filename); // Convert filename to Path to use extension
        let new_filename = format!(
            "{}({}){}",
            path.file_stem().unwrap_or_default().to_string_lossy(), // File name without extension
            counter,
            path.extension()
                .map_or(String::new(), |ext| format!(".{}", ext.to_string_lossy())) // Preserve the extension
        );
        trash_path = trash_dir.join(new_filename);
        counter += 1;
    }

    // Try to rename (move) the file to the trash directory
    fs::rename(source, &trash_path)?;

    Ok(())
}

