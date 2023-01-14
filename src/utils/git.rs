use std::path::{Path, PathBuf};

pub fn get_remote_url(config_file_path: &Path) -> String {
    let map = ini!(config_file_path.to_str().unwrap());

    for (name, children) in map.iter() {
        if name.starts_with("remote") {
            return children.get("url").unwrap().clone().unwrap();
        }
    }

    panic!("Cannot find remote url");
}

pub fn find_git_directory(directory: &Path) -> PathBuf {
    for parent in directory.ancestors() {
        let potential_git_dir = parent.join(".git");

        if potential_git_dir.is_dir() {
            return potential_git_dir;
        }
    }

    panic!("Not in a git repository");
}
