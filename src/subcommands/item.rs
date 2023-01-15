use std::{env, path::PathBuf};

use clap::Args;

use crate::{
    ado::organisation::{
        organisation::Organisation,
        project::repository::parse_remote_url::{parse_remote_url, ParsedRemoteUrl},
    },
    utils::git::{find_git_directory, get_remote_url},
};

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
    let pr = repository.get_pull_request_for_current_branch().await;

    if let None = pr {
        println!("There is no PR for the currently checked out branch");
        return ();
    }

    let pr = pr.unwrap();
    let work_items = pr.get_linked_work_items().await;

    if work_items.len() > 0 {
        let work_item_url = work_items[0].get_url();
        work_item_url.open_in_browser();
    }
}
