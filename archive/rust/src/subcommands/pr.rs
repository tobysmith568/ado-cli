use std::{env, path::PathBuf};

use clap::Args;

use crate::{
    ado::organisation::project::repository::Repository,
    cli::{
        cli_result::CliResult,
        prompt_yes_no::{prompt_yes_no, YesNoResult},
    },
};

#[derive(Args, Debug)]
pub struct Pr {
    /// The directory to run the command from. Defaults to the current working directory
    #[arg(short, long)]
    directory: Option<PathBuf>,

    /// The branch name to use. Defaults to the currently checked out branch
    #[arg(short, long)]
    branch: Option<String>,

    /// Pass this flag if you want to create a new pull request
    #[arg(short, long)]
    create: bool,
}

pub async fn run_pr_command(options: Pr, api_key: String) -> CliResult {
    let working_dir = options
        .directory
        .unwrap_or_else(|| env::current_dir().expect("Cannot access the current directory"));

    let repository = match Repository::parse_from_directory(&working_dir, api_key) {
        Ok(repo) => repo,
        Err(err) => return err.into_result(),
    };

    let branch_name = options
        .branch
        .unwrap_or_else(|| repository.get_current_branch());

    if options.create {
        handle_creating_a_new_pr(&repository, &branch_name);
        return CliResult::Success;
    }

    let pr_result = repository.get_pull_request_for_branch(&branch_name).await;

    let pr = match pr_result {
        Err(err) => return err.into_result(),
        Ok(Some(pr)) => pr,
        Ok(None) => {
            handle_no_pr_exists(&repository, &branch_name);
            return CliResult::Success;
        }
    };

    let pr_url = pr.get_url();

    pr_url.open_in_browser();

    CliResult::Success
}

fn handle_no_pr_exists(repository: &Repository, branch_name: &str) {
    let question = format!(
        "There is no open PR for branch {}. Would you like to open one?",
        branch_name
    );

    let should_create_new = prompt_yes_no(&question);

    if let YesNoResult::Yes = should_create_new {
        handle_creating_a_new_pr(repository, branch_name);
    }
}

fn handle_creating_a_new_pr(repository: &Repository, branch_name: &str) {
    let url = repository.get_create_pr_url_for_branch(branch_name);
    url.open_in_browser();
}
