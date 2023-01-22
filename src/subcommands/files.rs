use std::{env, path::PathBuf};

use clap::Args;

use crate::{ado::organisation::project::repository::Repository, cli::cli_result::CliResult};

#[derive(Args, Debug)]
pub struct Files {
    /// The directory to run the command from. Defaults to the current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,

    /// The branch name to use. Defaults to the currently checked out branch
    #[arg(short, long)]
    branch: Option<String>,
}

pub fn run_files_command(options: Files, api_key: String) -> CliResult {
    let working_dir = options
        .directory
        .unwrap_or_else(|| env::current_dir().expect("Cannot access the current directory"));

    let repository = match Repository::parse_from_directory(working_dir, api_key) {
        Ok(repo) => repo,
        Err(err) => return err.into_result(),
    };

    let branch_name = options
        .branch
        .unwrap_or_else(|| repository.get_current_branch());

    let files_url = repository.get_files_url_for_branch(&branch_name);
    files_url.open_in_browser();

    CliResult::Success
}
