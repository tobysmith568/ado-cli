use std::process::Command;

use super::repository::Repository;

pub fn get_current_branch(repository: &Repository) -> String {
    let command_result = Command::new("git")
        .current_dir(&repository.local_location)
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("Failed to get current git branch");

    String::from_utf8(command_result.stdout)
        .unwrap()
        .trim()
        .to_string()
}
