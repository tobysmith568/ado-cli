use std::{env, path::PathBuf};

use clap::Args;

use crate::utils::{
    ado_repo::AdoRepo,
    browser::open_url,
    git::{get_current_branch, get_remote_url},
    string::url_encode,
};

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

    let remote_url = get_remote_url(&working_dir);

    let AdoRepo {
        organisation_name,
        project_name,
        repository_name,
    } = AdoRepo::parse_from_remote_url(&remote_url);

    let branch_name = get_current_branch(&working_dir);
    let url_encoded_branch = url_encode(&branch_name);

    let files_page_url = format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_git/{repository_name}?version=GB{url_encoded_branch}"
        );

    open_url(&files_page_url);
}
