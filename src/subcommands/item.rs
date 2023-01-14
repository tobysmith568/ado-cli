use std::{env, path::PathBuf};

use clap::Args;

use crate::{ado_repo::AdoRepo, utils::browser::open_url};

#[derive(Args, Debug)]
pub struct Item {
    /// The current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,
}

pub async fn run_item_command(options: Item) {
    let working_dir = options
        .directory
        .unwrap_or_else(|| env::current_dir().expect("Cannot access the current directory"));

    let repo = AdoRepo::from_directory(&working_dir);

    let pr = repo.get_pr().await.unwrap();
    let work_items = pr.pr_work_items().await;

    if work_items.len() > 0 {
        open_url(&work_items[0].get_url());
    }
}
