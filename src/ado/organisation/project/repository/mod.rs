use std::path::{Path, PathBuf};

use crate::{
    ado::organisation::Organisation,
    cli::cli_error::CliError,
    utils::{
        git::{find_git_repository_root, get_remote_url},
        string::url_encode,
        url::Url,
    },
};

use self::{
    current_branch::get_current_branch,
    parse_remote_url::{parse_remote_url, ParsedRemoteUrl},
    pr_id::get_pr_id,
    pull_request::PullRequest,
};

use super::Project;

mod current_branch;
mod parse_remote_url;
mod pr_id;
mod pull_request;
mod repository_id;

pub struct Repository {
    pub name: String,
    pub local_location: PathBuf,
    pub project: Project,
}

impl Repository {
    pub fn new(project: Project, name: &str, local_location: &Path) -> Repository {
        Repository {
            name: name.to_string(),
            local_location: local_location.to_path_buf(),
            project,
        }
    }

    pub fn parse_from_directory(directory: &Path, api_key: String) -> Result<Repository, CliError> {
        let git_repository_root = find_git_repository_root(directory)?;

        let config_file_path = git_repository_root.join(".git").join("config");

        let remote_url = get_remote_url(&config_file_path)?;

        let parsed_remote_url = parse_remote_url(&remote_url)?;

        let ParsedRemoteUrl {
            organisation_name,
            project_name,
            repository_name,
            ..
        } = parsed_remote_url;

        let organisation = Organisation::new(&organisation_name, api_key);
        let project = organisation.get_project(&project_name);
        let repository = project.get_repository(&repository_name, &git_repository_root);

        Ok(repository)
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

    pub fn get_files_url_for_file_on_branch(
        &self,
        branch_name: &str,
        file_or_directory: &str,
        working_dir: &Path,
    ) -> Url {
        let project_name = &self.project.name;
        let organisation_name = &self.project.organisation.name;
        let repository_name = &self.name;

        let url_encoded_branch = url_encode(branch_name);

        let url_encoded_file_path = self.format_ado_file_path(file_or_directory, working_dir);

        let url_text = format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_git/{repository_name}?version=GB{url_encoded_branch}&path={url_encoded_file_path}"
        );

        Url::from(url_text)
    }

    pub async fn get_pull_request_for_branch(
        &self,
        branch_name: &str,
    ) -> Result<Option<PullRequest>, CliError> {
        let pr_id = get_pr_id(self, branch_name).await?;

        Ok(pr_id.map(|id| PullRequest::new(self, id)))
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

    fn format_ado_file_path(&self, file_path: &str, working_dir: &Path) -> String {
        let full_path = working_dir.join(file_path).canonicalize().unwrap();

        let repository_root = &self.local_location;

        let repository_root_segments = repository_root.components().count();

        let result: String = full_path
            .iter()
            .skip(repository_root_segments)
            .map(|seg| format!("/{}", seg.to_string_lossy()))
            .collect();

        dbg!(&result);
        url_encode(&result)
    }
}
