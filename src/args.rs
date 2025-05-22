use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true)]
#[command(
    version,
    about = "A modern take on the traditional rm command, written in Rust."
)]

pub struct Args {
    //
    #[arg(short = 'r', long, action = ArgAction::SetTrue)]
    pub recursive: bool,

    /// Display contents in trash
    #[arg(short = 'v', long, action = ArgAction::SetTrue)]
    pub view_trash: bool,

    /// Free-form positional arguments
    #[arg()]
    pub files: Vec<String>,
}

pub fn test() {
    println!("r has been called!");
}

pub fn view_trash() {
    println!("view_trash() called");
}
