use std::{env, path::PathBuf};

use clap::Args;

use crate::{ado_repo::AdoRepo, utils::browser::open_url};

#[derive(Args, Debug)]
pub struct Pr {
    /// The current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

pub async fn run_pr_command(options: Pr) {
    let working_dir = options
        .directory
        .unwrap_or_else(|| env::current_dir().expect("Cannot access the current directory"));

    let repo = AdoRepo::from_directory(&working_dir);

    let pr = repo.get_pr().await.unwrap();
    let pr_url = pr.pr_page_url().await.unwrap();

    open_url(&pr_url);
}
