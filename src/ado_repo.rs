use std::{path::PathBuf, process::Command};

use lazy_static::lazy_static;
use regex::Regex;

use crate::utils::{
    git::{find_git_directory, get_remote_url},
    string::url_encode,
};

#[derive(Debug)]
pub struct AdoRepo {
    pub organisation_name: String,
    pub project_name: String,
    pub repository_name: String,

    directory: String,
}

impl AdoRepo {
    fn new(
        organisation_name: &str,
        project_name: &str,
        repository_name: &str,
        directory: &str,
    ) -> AdoRepo {
        AdoRepo {
            organisation_name: organisation_name.to_string(),
            project_name: project_name.to_string(),
            repository_name: repository_name.to_string(),
            directory: directory.to_string(),
        }
    }

    pub fn from_directory(directory: &PathBuf) -> AdoRepo {
        let git_directory = find_git_directory(&directory);
        let config_file_path = git_directory.join("config");
        let remote_url = get_remote_url(&config_file_path);

        if remote_url.starts_with("http") {
            parse_from_http_remote_url(&remote_url, &git_directory)
        } else {
            parse_from_ssh_remote_url(&remote_url, &git_directory)
        }
    }

    pub fn files_page_url(&self) -> String {
        let url_encoded_branch = url_encode(&self.current_branch());

        let AdoRepo {
            organisation_name,
            project_name,
            repository_name,
            ..
        } = self;

        format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_git/{repository_name}?version=GB{url_encoded_branch}"
        )
    }

    pub fn current_branch(&self) -> String {
        let command_result = Command::new("git")
            .current_dir(&self.directory)
            .arg("branch")
            .arg("--show-current")
            .output()
            .expect("Failed to get current git branch");

        String::from_utf8(command_result.stdout)
            .unwrap()
            .trim()
            .to_string()
    }
}

fn parse_from_http_remote_url(remote_url: &str, git_directory: &PathBuf) -> AdoRepo {
    lazy_static! {
        static ref HTTP_URL_REGEX: Regex =
            Regex::new(r"^https://.*?@dev.azure.com/(.*?)/(.*?)/_git/(.*?)$").unwrap();
    }

    let result = HTTP_URL_REGEX
        .captures(remote_url)
        .expect("Cannot parse remote url");

    let organisation_name = &result[1];
    let project_name = &result[2];
    let repository_name = &result[3];
    let git_directory = git_directory.to_str().unwrap();

    AdoRepo::new(
        organisation_name,
        project_name,
        repository_name,
        git_directory,
    )
}

fn parse_from_ssh_remote_url(remote_url: &str, git_directory: &PathBuf) -> AdoRepo {
    lazy_static! {
        static ref SSH_URL_REGEX: Regex =
            Regex::new(r"^git@ssh.dev.azure.com:v\d/(.*?)/(.*?)/(.*?)$").unwrap();
    }

    let result = SSH_URL_REGEX
        .captures(remote_url)
        .expect("Cannot parse remote url");

    let organisation_name = &result[1];
    let project_name = &result[2];
    let repository_name = &result[3];
    let git_directory = git_directory.to_str().unwrap();

    AdoRepo::new(
        organisation_name,
        project_name,
        repository_name,
        git_directory,
    )
}
