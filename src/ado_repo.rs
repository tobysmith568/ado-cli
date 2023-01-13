use lazy_static::lazy_static;
use regex::Regex;
use urlencoding::encode;

use crate::browser_utils::open_url;

#[derive(Debug)]
pub struct AdoRepo {
    organisation_name: String,
    project_name: String,
    repository_name: String,
    branch_name: String,
}

impl AdoRepo {
    pub fn new(
        organisation_name: &str,
        project_name: &str,
        repository_name: &str,
        branch_name: Option<&str>,
    ) -> AdoRepo {
        AdoRepo {
            organisation_name: organisation_name.to_string(),
            project_name: project_name.to_string(),
            repository_name: repository_name.to_string(),
            branch_name: String::from(branch_name.unwrap_or("main")),
        }
    }

    pub fn from_remote_url_and_branch(remote_url: &str, branch: &str) -> AdoRepo {
        if remote_url.starts_with("http") {
            get_repo_from_http(remote_url, Some(branch))
        } else {
            get_repo_from_ssh(remote_url, Some(branch))
        }
    }

    pub fn open_files_page(&self) {
        let AdoRepo {
            organisation_name,
            project_name,
            repository_name,
            branch_name,
        } = self;

        let url_encoded_branch = encode(&branch_name).to_string();

        let files_page_url = format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_git/{repository_name}?version=GB{url_encoded_branch}"
        );

        open_url(&files_page_url);
    }
}

fn get_repo_from_http(remote_url: &str, branch_name: Option<&str>) -> AdoRepo {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^https://.*?@dev.azure.com/(.*?)/(.*?)/_git/(.*?)$").unwrap();
    }

    let result = RE.captures(remote_url).expect("Cannot parse remote url");

    let organisation_name = &result[1];
    let project_name = &result[2];
    let repository_name = &result[3];

    AdoRepo::new(
        organisation_name,
        project_name,
        repository_name,
        branch_name,
    )
}

fn get_repo_from_ssh(remote_url: &str, branch_name: Option<&str>) -> AdoRepo {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^git@ssh.dev.azure.com:v\d/(.*?)/(.*?)/(.*?)$").unwrap();
    }

    let result = RE.captures(remote_url).expect("Cannot parse remote url");

    let organisation_name = &result[1];
    let project_name = &result[2];
    let repository_name = &result[3];

    AdoRepo::new(
        organisation_name,
        project_name,
        repository_name,
        branch_name,
    )
}
