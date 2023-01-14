use std::{env, path::PathBuf};

use clap::Args;

use crate::{ado_repo::AdoRepo, utils::browser::open_url};

#[derive(Args, Debug)]
pub struct Files {
    /// The current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

pub fn run_files_command(options: Files) {
    let working_dir = options
        .directory
        .unwrap_or_else(|| env::current_dir().expect("Cannot access the current directory"));

    let repo = AdoRepo::from_directory(&working_dir);

    let files_page_url = repo.files_page_url();

    open_url(&files_page_url);
}
