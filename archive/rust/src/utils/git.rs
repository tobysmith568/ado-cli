use ini::{Ini, Properties};
use std::path::{Path, PathBuf};

use crate::cli::cli_error::CliError;

pub fn get_remote_url(config_file_path: &Path) -> Result<String, CliError> {
    let conf_result = Ini::load_from_file(config_file_path);

    let conf = match conf_result {
        Ok(conf) => conf,
        Err(err) => {
            let message = format!(
                "Cannot open or find the config file at {}, because: {}",
                config_file_path.to_string_lossy(),
                err
            );

            return CliError::err(&message);
        }
    };

    let remote_section = match find_remote_section(&conf) {
        Some(section) => section,
        None => return CliError::err("Cannot find a 'remote' section in the git config file"),
    };

    let url = remote_section.get("url");

    match url {
        Some(url) => Ok(url.to_string()),
        None => CliError::err("Cannot find remote url in the git config file"),
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

pub fn find_git_repository_root(directory: &Path) -> Result<PathBuf, CliError> {
    for parent in directory.ancestors() {
        let potential_git_dir = parent.join(".git");

        if potential_git_dir.is_dir() {
            return Ok(parent.to_path_buf());
        }
    }

    CliError::err("Not in a git repository")
}
