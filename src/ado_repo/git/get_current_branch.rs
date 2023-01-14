use std::process::Command;

use crate::ado_repo::AdoRepo;

pub fn get_current_branch(ado_repo: &AdoRepo) -> String {
    let command_result = Command::new("git")
        .current_dir(&ado_repo.directory)
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("Failed to get current git branch");

    String::from_utf8(command_result.stdout)
        .unwrap()
        .trim()
        .to_string()
}
