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
mod view;

use args::Args;
use clap::Parser;
use rayon::prelude::*;

// Interm program name.
const PROGRAM_NAME: &str = "rrm";

fn main() {
    // Parse the command line arguments
    let args = Args::parse();
    let mut allow_dir_removal: bool = false;

    // Check if the 'recursive' flag was passed.
    if args.recursive {
        allow_dir_removal = true;
    } else if args.view_trash {
        view::list_trash_contents();
        return;
    }

    // If no files/dirs were specified, inform user and exit.
    if args.files.len() < 1 {
        println!("{}: missing operand", PROGRAM_NAME);
        println!("Try '{} --help' for more information.", PROGRAM_NAME);
        std::process::exit(1)
    }

    // If working with fewer than three files, process them serially, otherwise process them in
    // parallel.
    if args.files.len() < 3 {
        process_files(args.files, allow_dir_removal);
    } else {
        process_files_parallel(args.files, allow_dir_removal);
    }
}

fn process_files(files: Vec<String>, allow_dir_removal: bool) {
    files.iter().for_each(|arg| {
        if let Err(e) = mv::move_to_trash(arg, allow_dir_removal) {
            eprintln!("Error moving file '{}' to trash: {}", arg, e);
        }
    });
}

fn process_files_parallel(files: Vec<String>, allow_dir_removal: bool) {
    files.par_iter().for_each(|arg| {
        if let Err(e) = mv::move_to_trash(arg, allow_dir_removal) {
            eprintln!("Error moving file '{}' to trash: {}", arg, e);
        }
    });
}
