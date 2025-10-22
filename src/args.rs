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
use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(
    version = "0.2.3",
    author = "Robert Pellegrin",
    about = "A modern take on the traditional rm utility, written in Rust.
        \nAuthor: Robert Pellegrin"
)]

pub struct Args {
    /// remove directories and their contents recursively
    #[arg(short = 'r', long, action = ArgAction::SetTrue)]
    pub recursive: bool,

    /// Display contents in trash
    #[arg(long, action = ArgAction::SetTrue)]
    pub view_trash: bool,

    /// Empty the trash
    #[arg(long, action = ArgAction::SetTrue)]
    pub empty: bool, 

    /// List of files or directories to send to trash
    #[arg()]
    pub files: Vec<String>,
}
