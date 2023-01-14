use std::{env, path::PathBuf};

use clap::Args;

#[derive(Args, Debug)]
pub struct Item {
    /// The current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

pub fn run_item_command(options: Item) {
    let working_dir = options
        .directory
        .unwrap_or_else(|| env::current_dir().expect("Cannot access the current directory"));

    dbg!(working_dir);
}
