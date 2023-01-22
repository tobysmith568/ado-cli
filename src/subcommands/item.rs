use std::{env, path::PathBuf};

use clap::Args;

use crate::{
    ado::organisation::{
        project::repository::parse_remote_url::{parse_remote_url, ParsedRemoteUrl},
        Organisation,
    },
    cli::{cli_error::CliError, cli_result::CliResult},
    utils::git::{find_git_repository_root, get_remote_url},
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

    let git_repository_root = match find_git_repository_root(&working_dir) {
        Ok(root) => root,
        Err(err) => return err.into_result(),
    };

    let config_file_path = git_repository_root.join(".git").join("config");

    let remote_url = match get_remote_url(&config_file_path) {
        Ok(url) => url,
        Err(err) => return err.into_result(),
    };

    let parsed_remote_url = match parse_remote_url(&remote_url) {
        Ok(url) => url,
        Err(err) => return err.into_result(),
    };

    let ParsedRemoteUrl {
        organisation_name,
        project_name,
        repository_name,
        ..
    } = parsed_remote_url;

    let organisation = Organisation::new(&organisation_name, api_key);
    let project = organisation.get_project(&project_name);
    let repository = project.get_repository(&repository_name, &git_repository_root);

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
