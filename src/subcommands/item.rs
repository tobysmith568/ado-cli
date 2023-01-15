use std::{env, path::PathBuf};

use clap::Args;

use crate::{
    ado::organisation::{
        project::repository::parse_remote_url::{parse_remote_url, ParsedRemoteUrl},
        Organisation,
    },
    utils::git::{find_git_directory, get_remote_url},
};

#[derive(Args, Debug)]
pub struct Item {
    /// The directory to run the command from. Defaults to the current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,

    /// The branch name to use. Defaults to the currently checked out branch
    #[arg(short, long)]
    branch: Option<String>,
}

pub async fn run_item_command(options: Item) {
    let working_dir = options
        .directory
        .unwrap_or_else(|| env::current_dir().expect("Cannot access the current directory"));

    let git_directory = find_git_directory(&working_dir);
    let config_file_path = git_directory.join("config");
    let remote_url = get_remote_url(&config_file_path);
    let ParsedRemoteUrl {
        organisation_name,
        project_name,
        repository_name,
        ..
    } = parse_remote_url(&remote_url);

    let organisation = Organisation::new(&organisation_name);
    let project = organisation.get_project(&project_name);
    let repository = project.get_repository(&repository_name, &working_dir);

    let branch_name = options
        .branch
        .unwrap_or_else(|| repository.get_current_branch());

    let pr = repository.get_pull_request_for_branch(&branch_name).await;

    if pr.is_none() {
        println!(
            "There is no open PR for the branch {}; so a linked PBI/Bug/etc. could not be discovered.",
            &branch_name
        );
        return;
    }

    let pr = pr.unwrap();
    let work_items = pr.get_linked_work_items().await;

    if !work_items.is_empty() {
        let work_item_url = work_items[0].get_url();
        work_item_url.open_in_browser();
    }
}
