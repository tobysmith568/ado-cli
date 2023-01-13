use std::{env, ops::Index, path::Path};

use crate::{
    ado_repo::AdoRepo,
    git_utils::{get_current_branch, get_remote_url},
};

mod ado_repo;
mod browser_utils;
mod git_utils;

#[macro_use]
extern crate ini;

fn main() {
    let cwd = env::current_dir().expect("Cannot access the current directory");

    let git_dir = git_utils::find_git_directory(&cwd).expect("Not in a git repository");
    let config_file_path = git_dir.join("config");
    let remote_url =
        get_remote_url(&config_file_path).expect("Could not find remote url in git config");

    dbg!(&remote_url);

    let git_branch = get_current_branch(&git_dir);

    dbg!(&git_branch);

    let repo = AdoRepo::from_remote_url_and_branch(&remote_url, &git_branch);

    dbg!(&repo);

    // repo.open_files_page();
}
