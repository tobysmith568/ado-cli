use std::path::{Path, PathBuf};
use std::process::Command;

pub fn get_remote_url(path: &Path) -> String {
    let git_directory = find_git_directory(path);
    let config_file = git_directory.join("config");

    let map = ini!(config_file.to_str().unwrap());

    for (name, children) in map.iter() {
        if name.starts_with("remote") {
            return children.get("url").unwrap().clone().unwrap();
        }
    }

    panic!("Cannot find remote url");
}

fn find_git_directory(path: &Path) -> PathBuf {
    for parent in path.ancestors() {
        let potential_git_dir = parent.join(".git");

        if potential_git_dir.is_dir() {
            return potential_git_dir;
        }
    }

    panic!("Not in a git repository");
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
