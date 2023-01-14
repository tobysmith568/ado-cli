use std::path::PathBuf;

use lazy_static::lazy_static;
use regex::Regex;

use super::AdoRepo;

pub fn parse_from_http_remote_url(remote_url: &str, git_directory: &PathBuf) -> AdoRepo {
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

pub fn parse_from_ssh_remote_url(remote_url: &str, git_directory: &PathBuf) -> AdoRepo {
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
