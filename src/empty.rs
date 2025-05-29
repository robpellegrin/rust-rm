///
/// =====================================================================
/// Project Name: rust rm
/// Description: An enhanced version of the common rm utility.
/// Author: Robert Pellegrin
/// Date: 2025-05-28
/// Version: 0.0.1
/// License: MIT
/// Repository:
/// =====================================================================
///
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Deletes all contents inside the given directory (but not the directory itself).
fn delete_directory_contents(dir: &Path) -> io::Result<()> {
    if dir.exists() && dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                fs::remove_dir_all(&path)?;
            } else {
                fs::remove_file(&path)?;
            }
        }
    }
    Ok(())
}

/// Prompts the user for a yes/no confirmation.
/// Returns true if the user enters 'y' or 'Y'.
fn confirm(prompt: &str) -> io::Result<bool> {
    print!("{} [y/N]: ", prompt);
    io::stdout().flush()?; // Make sure prompt shows up before input

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let response = input.trim().to_lowercase();

    Ok(response == "y")
}

/// Empties the user's trash directory located at ~/.local/share/Trash/{files,info}
pub fn empty_trash() -> io::Result<()> {
    if !confirm("Are you sure you want to permanently delete all files in the trash?")? {
        return Ok(()); // Do nothing if user says no
    }

    let home = env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME env var not set"))?;
    let trash_base = PathBuf::from(home).join(".local/share/Trash");

    let files_dir = trash_base.join("files");
    let info_dir = trash_base.join("info");

    delete_directory_contents(&files_dir)?;
    delete_directory_contents(&info_dir)?;

    Ok(())
}
