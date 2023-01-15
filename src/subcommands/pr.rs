use std::{env, path::PathBuf};

use clap::Args;

use crate::{
    ado::organisation::{
        organisation::Organisation,
        project::repository::{
            parse_remote_url::{parse_remote_url, ParsedRemoteUrl},
            repository::Repository,
        },
    },
    cli::prompt_yes_no::{prompt_yes_no, YesNoResult},
    utils::git::{find_git_directory, get_remote_url},
};

#[derive(Args, Debug)]
pub struct Pr {
    /// The directory to run the command from. Defaults to the current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,

    /// The branch name to use. Defaults to the currently checked out branch
    #[arg(short, long)]
    branch: Option<String>,
}

pub async fn run_pr_command(options: Pr) {
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

    let branch_name = options.branch.unwrap_or(repository.get_current_branch());

    let pr = repository.get_pull_request_for_branch(&branch_name).await;

    if let None = pr {
        return handle_no_pr_exists(&repository, &branch_name);
    }

    let pr = pr.unwrap();
    let pr_url = pr.get_url();

    pr_url.open_in_browser();
}

fn handle_no_pr_exists(repository: &Repository, branch_name: &str) {
    let question = format!(
        "There is no open PR for branch {}. Would you like to open one?",
        branch_name
    );

    let should_create_new = prompt_yes_no(&question);

    if let YesNoResult::No = should_create_new {
        return ();
    }

    let url = repository.get_create_pr_url_for_branch(branch_name);
    url.open_in_browser();
}
