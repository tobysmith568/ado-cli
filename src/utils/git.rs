use ini::{Ini, Properties};
use std::path::{Path, PathBuf};

pub fn get_remote_url(config_file_path: &Path) -> String {
    let conf_result = Ini::load_from_file(config_file_path);

    if let Err(err) = conf_result {
        panic!(
            "Cannot open or find the config file at {}, because: {}",
            config_file_path.to_str().unwrap(),
            err
        )
    }

    let conf = conf_result.unwrap();

    let remote_section =
        find_remote_section(&conf).expect("Cannot find a 'remote' section in the git config file");

    let url = remote_section.get("url");

    match url {
        Some(url) => url.to_string(),
        None => panic!("Cannot find remote url in the git config file"),
    }
}

fn find_remote_section(ini: &Ini) -> Option<&Properties> {
    for section in ini.sections() {
        let section_name = match section {
            Some(name) => name,
            None => continue,
        };

        if section_name.starts_with("remote") {
            return ini.section(Some(section_name));
        }
    }

    None
}

pub fn find_git_repository_root(directory: &Path) -> PathBuf {
    for parent in directory.ancestors() {
        let potential_git_dir = parent.join(".git");

        if potential_git_dir.is_dir() {
            return parent.to_path_buf();
        }
    }

    panic!("Not in a git repository");
}
