use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
pub struct AdoRepo {
    pub organisation_name: String,
    pub project_name: String,
    pub repository_name: String,
}

impl AdoRepo {
    fn new(organisation_name: &str, project_name: &str, repository_name: &str) -> AdoRepo {
        AdoRepo {
            organisation_name: organisation_name.to_string(),
            project_name: project_name.to_string(),
            repository_name: repository_name.to_string(),
        }
    }

    pub fn parse_from_remote_url(remote_url: &str) -> AdoRepo {
        if remote_url.starts_with("http") {
            parse_from_http_remote_url(remote_url)
        } else {
            parse_from_ssh_remote_url(remote_url)
        }
    }
}

fn parse_from_http_remote_url(remote_url: &str) -> AdoRepo {
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

    AdoRepo::new(organisation_name, project_name, repository_name)
}

fn parse_from_ssh_remote_url(remote_url: &str) -> AdoRepo {
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

    AdoRepo::new(organisation_name, project_name, repository_name)
}
