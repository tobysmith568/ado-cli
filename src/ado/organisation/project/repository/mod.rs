use std::path::{Path, PathBuf};

use crate::utils::{string::url_encode, url::Url};

use self::{current_branch::get_current_branch, pr_id::get_pr_id, pull_request::PullRequest};

use super::Project;

mod current_branch;
pub mod parse_remote_url;
mod pr_id;
mod pull_request;
mod repository_id;

pub struct Repository<'a> {
    pub name: String,
    pub local_location: PathBuf,
    pub project: &'a Project<'a>,
}

impl<'a> Repository<'a> {
    pub fn new(project: &'a Project, name: &str, local_location: &Path) -> Repository<'a> {
        Repository {
            name: name.to_string(),
            local_location: local_location.to_path_buf(),
            project,
        }
    }

    pub fn get_current_branch(&self) -> String {
        get_current_branch(self)
    }

    pub fn get_files_url_for_branch(&self, branch_name: &str) -> Url {
        let project_name = &self.project.name;
        let organisation_name = &self.project.organisation.name;
        let repository_name = &self.name;

        let url_encoded_branch = url_encode(branch_name);

        let url_text = format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_git/{repository_name}?version=GB{url_encoded_branch}"
        );

        Url::from(url_text)
    }

    pub async fn get_pull_request_for_branch(&self, branch_name: &str) -> Option<PullRequest> {
        let pr_id = get_pr_id(self, branch_name).await?;

        Some(PullRequest::new(self, pr_id))
    }

    pub fn get_create_pr_url_for_branch(&self, branch_name: &str) -> Url {
        let organisation_name = &self.project.organisation.name;
        let project_name = &self.project.name;
        let repository_name = &self.name;

        let url_encoded_branch_name = url_encode(branch_name);

        let url_text = format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_git/{repository_name}/pullrequestcreate?sourceRef={url_encoded_branch_name}"
        );

        Url::from(url_text)
    }
}
