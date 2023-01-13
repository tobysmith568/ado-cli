use std::path::{Path, PathBuf};
use std::process::Command;

pub fn find_git_directory(path: &Path) -> Option<PathBuf> {
    for parent in path.ancestors() {
        let potential_git_dir = parent.join(".git");

        if potential_git_dir.is_dir() {
            return Some(potential_git_dir);
        }
    }

    return None;
}

pub fn get_remote_url(git_directory: &PathBuf) -> Option<String> {
    let map = ini!(git_directory.to_str().unwrap());

    for (name, children) in map.iter() {
        if name.starts_with("remote") {
            return children.get("url").unwrap().clone();
        }
    }

    return None;
}

pub fn get_current_branch(directory: &PathBuf) -> String {
    let command_result = Command::new("git")
        .current_dir(directory.to_str().unwrap())
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("Failed to get current git branch");

    return String::from_utf8(command_result.stdout)
        .unwrap()
        .trim()
        .to_string();
}
