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
/// - Handle command line arguments to view, empty and list trash contents.
/// - Store metadata along with deleted files to make restoring easier.
/// - Restore files from trash using CLI
/// - View trash contents via CLI.
/// - Config file to allow user to specify custom path to trash, size limitations, etc.
/// - Integration with GUI trash bins (e.g., Dolphin, Nautilus) for restoration.
mod args;
mod mv;
mod utils;

use args::Args;
use clap::Parser;

fn main() {
    // Parse the command line arguments
    let args = Args::parse();
    let mut allow_dir_removal = false;

    // Check if the 'recursive' flag was passed, and call 'test' if true
    if args.recursive {
        allow_dir_removal = true;
    } else if args.view_trash{
        args::view_trash();
    }

    // let args: Vec<String> = env::args().collect();

    // Loop over each argument (excluding the first one, which is the program name)
    for arg in &args.files {
        if let Err(e) = mv::move_to_trash(arg, allow_dir_removal) {
            eprintln!("Error moving file '{}' to trash: {}", arg, e);
        }
    }
}
