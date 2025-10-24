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

/// Deletes all contents inside the given directory (but not the directory
/// itself).
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

    // Make sure prompt shows up before input
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let response = input.trim().to_lowercase();

    Ok(response == "y")
}

/// Count entries in a directory (non-recursive)
fn count_entries_in_dir<P: AsRef<std::path::Path>>(path: P) -> io::Result<usize> {
    Ok(fs::read_dir(path)?.count())
}

/// Empties the user's trash directory located at:
/// ~/.local/share/Trash/{files,info}
pub fn empty_trash() -> io::Result<()> {
    let home = env::var("HOME")
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "HOME env var not set"))?;
    let trash_base = PathBuf::from(home).join(".local/share/Trash");

    let files_dir = trash_base.join("files");
    let info_dir = trash_base.join("info");

    let count = count_entries_in_dir(&info_dir)?;

    // Prompt the user with the count included
    let prompt = format!("Permanently delete all {} file(s) in the trash?", count);

    if !confirm(&prompt)? {
        println!("Cancelled");
        return Ok(());
    }

    delete_directory_contents(&files_dir)?;
    delete_directory_contents(&info_dir)?;

    Ok(())
}
