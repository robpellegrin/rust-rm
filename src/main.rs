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

fn main() {
    let args = Args::parse();

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

    // If no files/dirs were specified, inform user and exit.
    if args.files.len() < 1 {
        view::list_trash_contents_table();
        return;
    }

    if args.interactive {
        process_files_serial(args.files, &Args::parse());
    } else {
        process_files_parallel(args.files, &Args::parse());
    }
}

fn process_files_parallel(files: Vec<String>, args: &Args) {
    files.par_iter().for_each(|arg| {
        if let Err(e) = mv::move_to_trash(arg, args) {
            eprintln!("rrm: cannot remove '{}': {}", arg, e);
        }
    });
}

fn process_files_serial(files: Vec<String>, args: &Args) {
    eprintln!("WARNING: interactive flag disabled parallelism");
    files.iter().for_each(|arg| {
        if let Err(e) = mv::move_to_trash(arg, args) {
            eprintln!("rrm: cannot remove '{}': {}", arg, e);
        }
    });
}
