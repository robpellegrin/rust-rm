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
    version = "0.2.8",
    author = "Robert Pellegrin",
    about = "A modern take on the traditional rm utility, written in Rust.
        \nAuthor: Robert Pellegrin"
)]

pub struct Args {
    /// remove directories and their contents recursively
    #[arg(short = 'r', long, action = ArgAction::SetTrue)]
    pub recursive: bool,

    /// list contents of trash directory
    #[arg(long, action = ArgAction::SetTrue)]
    pub view_trash: bool,

    /// explain what is being done.
    #[arg(short = 'v', long, action = ArgAction::SetTrue)]
    pub verbose: bool,

    /// prompt before every removal
    #[arg(short = 'i', action = ArgAction::SetTrue)]
    pub interactive: bool,

    /// permanently delete all files in the trash directory
    #[arg(long, action = ArgAction::SetTrue)]
    pub empty: bool,

    /// remove directories and their contents recursively
    #[arg(short = 's', long, action = ArgAction::SetTrue)]
    pub skip_trash: bool,

    /// list of files/directories to send to trash
    #[arg()]
    pub files: Vec<String>,
}
