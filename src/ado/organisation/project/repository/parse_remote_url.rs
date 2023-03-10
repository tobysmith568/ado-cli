use lazy_static::lazy_static;
use regex::Regex;

use crate::cli::cli_error::CliError;

pub struct ParsedRemoteUrl {
    pub organisation_name: String,
    pub project_name: String,
    pub repository_name: String,
}

pub fn parse_remote_url(remote_url: &str) -> Result<ParsedRemoteUrl, CliError> {
    if remote_url.starts_with("http") {
        parse_http_remote_url(remote_url)
    } else {
        parse_ssh_remote_url(remote_url)
    }
}

fn parse_http_remote_url(remote_url: &str) -> Result<ParsedRemoteUrl, CliError> {
    lazy_static! {
        static ref HTTP_URL_REGEX: Regex =
            Regex::new(r"^https://.*?@dev.azure.com/(.*?)/(.*?)/_git/(.*?)$").unwrap();
    }

    let regex_result = HTTP_URL_REGEX.captures(remote_url);

    let result = match regex_result {
        Some(r) => r,
        None => return CliError::err("Cannot parse remote url"),
    };

    let organisation_name = &result[1];
    let project_name = &result[2];
    let repository_name = &result[3];

    Ok(ParsedRemoteUrl {
        organisation_name: organisation_name.to_string(),
        project_name: project_name.to_string(),
        repository_name: repository_name.to_string(),
    })
}

fn parse_ssh_remote_url(remote_url: &str) -> Result<ParsedRemoteUrl, CliError> {
    lazy_static! {
        static ref SSH_URL_REGEX: Regex =
            Regex::new(r"^git@ssh.dev.azure.com:v\d/(.*?)/(.*?)/(.*?)$").unwrap();
    }

    let regex_result = SSH_URL_REGEX.captures(remote_url);

    let result = match regex_result {
        Some(r) => r,
        None => return CliError::err("Cannot parse remote url"),
    };

    let organisation_name = &result[1];
    let project_name = &result[2];
    let repository_name = &result[3];

    Ok(ParsedRemoteUrl {
        organisation_name: organisation_name.to_string(),
        project_name: project_name.to_string(),
        repository_name: repository_name.to_string(),
    })
}
