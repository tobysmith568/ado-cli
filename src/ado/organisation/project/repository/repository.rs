use std::path::PathBuf;

use crate::{
    ado::organisation::project::project::Project,
    utils::{string::url_encode, url::Url},
};

use super::{
    current_branch::get_current_branch, get_pr_id::get_pr_id,
    pull_request::pull_request::PullRequest,
};

pub struct Repository<'a> {
    pub name: String,
    pub local_location: PathBuf,
    pub project: &'a Project<'a>,
}

impl<'a> Repository<'a> {
    pub fn new(project: &'a Project, name: &str, local_location: &PathBuf) -> Repository<'a> {
        Repository {
            name: name.to_string(),
            local_location: local_location.to_path_buf(),
            project,
        }
    }

    pub fn get_current_branch(&self) -> String {
        get_current_branch(self)
    }

    pub fn get_files_url_for_current_branch(&self) -> Url {
        let current_branch = get_current_branch(self);
        self.get_files_url_for_branch(&current_branch)
    }

    pub async fn get_pull_request_for_current_branch(&self) -> Option<PullRequest> {
        let pr_id = get_pr_id(self).await?;

        Some(PullRequest::new(self, pr_id))
    }

    fn get_files_url_for_branch(&self, branch_name: &str) -> Url {
        let project_name = &self.project.name;
        let organisation_name = &self.project.organisation.name;
        let repository_name = &self.name;

        let url_encoded_branch = url_encode(branch_name);

        let url_text = format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_git/{repository_name}?version=GB{url_encoded_branch}"
        );

        Url::from(url_text)
    }
}
