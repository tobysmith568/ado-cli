use std::path::PathBuf;

use crate::utils::{
    git::{find_git_directory, get_remote_url},
    string::url_encode,
};

use self::{
    ado::repository_id::{
        get_repository_id_from_api, get_repository_id_from_disk, save_repository_id_to_disk,
    },
    ado_pr::AdoPr,
    git::get_current_branch::get_current_branch,
    parse_from_remote_url::{parse_from_http_remote_url, parse_from_ssh_remote_url},
    pr_id::get_pr_id,
};

mod ado;
mod ado_pr;
mod ado_work_item;
mod git;
mod parse_from_remote_url;
mod pr_id;

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

    async fn get_repository_id(&self) -> String {
        let repository_id = get_repository_id_from_disk(&self);

        if let Some(id) = repository_id {
            return id;
        }

        let repository_id = get_repository_id_from_api(&self).await;
        save_repository_id_to_disk(&self, &repository_id);
        repository_id
    }

    pub fn current_branch(&self) -> String {
        get_current_branch(self)
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

    pub async fn get_pr(&self) -> Option<AdoPr> {
        let pr_id = get_pr_id(&self).await?;

        Some(AdoPr::from(&self, pr_id))
    }
}
