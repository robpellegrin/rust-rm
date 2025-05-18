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

    // Join the trash directory with the filename
    let trash_path = trash_dir.join(filename);

    // Try to rename (move) the file to the trash directory
    fs::rename(source, &trash_path)?;

    Ok(())
}
