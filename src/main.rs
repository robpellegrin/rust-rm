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
mod args;
mod empty;
mod mv;
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

    if args.view_trash {
        view::list_trash_contents_table();
        return;
    }

    if args.empty {
        if let Err(e) = empty::empty_trash() {
            eprintln!("Failed to empty trash: {}", e);
        }
        return;
    }

    // Check if the 'recursive' flag was passed.
    if args.recursive {
        allow_dir_removal = true;
    }

    // If no files/dirs were specified, inform user and exit.
    if args.files.len() < 1 {
        println!("{}: missing operand", PROGRAM_NAME);
        println!("Try '{} --help' for more information.", PROGRAM_NAME);
        std::process::exit(1)
    }

    process_files_parallel(args.files, allow_dir_removal);
}

fn process_files_parallel(files: Vec<String>, allow_dir_removal: bool) {
    files.par_iter().for_each(|arg| {
        if let Err(e) = mv::move_to_trash(arg, allow_dir_removal) {
            eprintln!("rrm: cannot remove '{}': {}", arg, e);
        }
    });
}
