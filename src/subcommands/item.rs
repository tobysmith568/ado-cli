use std::{env, path::PathBuf};

use clap::Args;

use crate::{
    ado::organisation::project::repository::Repository,
    cli::{cli_error::CliError, cli_result::CliResult},
};

#[derive(Args, Debug)]
pub struct Item {
    /// The directory to run the command from. Defaults to the current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,

    /// The branch name to use. Defaults to the currently checked out branch
    #[arg(short, long)]
    branch: Option<String>,

    /// The ID to the PBI/Bug/etc. to use. Defaults to the those attached to the PR for the current branch
    #[arg(long)]
    id: Option<String>,
}

pub async fn run_item_command(options: Item, api_key: String) -> CliResult {
    let working_dir = options
        .directory
        .unwrap_or_else(|| env::current_dir().expect("Cannot access the current directory"));

    let repository = match Repository::parse_from_directory(working_dir, api_key) {
        Ok(repo) => repo,
        Err(err) => return err.into_result(),
    };

    let project = &repository.project;

    if let Some(id) = options.id {
        let work_item = project.get_work_item(id);
        let work_item_url = work_item.get_url();
        work_item_url.open_in_browser();
        return CliResult::Success;
    }

    let branch_name = options
        .branch
        .unwrap_or_else(|| repository.get_current_branch());

    let pr_result = repository.get_pull_request_for_branch(&branch_name).await;

    let pr = match pr_result {
        Err(err) => return err.into_result(),
        Ok(Some(pr)) => pr,
        Ok(None) => {
            let message = format!("There is no open PR for the branch {}; so a linked PBI/Bug/etc. could not be discovered.", branch_name);
            return CliError::new(&message).into_result();
        }
    };

    let work_items = match pr.get_linked_work_items().await {
        Ok(items) => items,
        Err(err) => return err.into_result(),
    };

    if !work_items.is_empty() {
        let work_item_url = work_items[0].get_url();
        work_item_url.open_in_browser();
    }

    CliResult::Success
}
