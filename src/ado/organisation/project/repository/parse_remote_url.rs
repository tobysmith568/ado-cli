use lazy_static::lazy_static;
use regex::Regex;

pub struct ParsedRemoteUrl {
    pub organisation_name: String,
    pub project_name: String,
    pub repository_name: String,
}

pub fn parse_remote_url(remote_url: &str) -> ParsedRemoteUrl {
    if remote_url.starts_with("http") {
        parse_http_remote_url(remote_url)
    } else {
        parse_ssh_remote_url(remote_url)
    }
}

fn parse_http_remote_url(remote_url: &str) -> ParsedRemoteUrl {
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

    ParsedRemoteUrl {
        organisation_name: organisation_name.to_string(),
        project_name: project_name.to_string(),
        repository_name: repository_name.to_string(),
    }
}

pub fn parse_ssh_remote_url(remote_url: &str) -> ParsedRemoteUrl {
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

    ParsedRemoteUrl {
        organisation_name: organisation_name.to_string(),
        project_name: project_name.to_string(),
        repository_name: repository_name.to_string(),
    }
}
