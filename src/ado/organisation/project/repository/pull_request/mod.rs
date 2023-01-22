use crate::{
    ado::organisation::project::work_item::WorkItem, cli::cli_error::CliError, utils::url::Url,
};

use self::work_items::get_work_items;

use super::Repository;

pub mod work_items;

pub struct PullRequest<'a> {
    pub id: i32,
    pub repository: &'a Repository<'a>,
}

impl<'a> PullRequest<'a> {
    pub fn new(repository: &'a Repository, id: i32) -> PullRequest<'a> {
        PullRequest { id, repository }
    }

    pub fn get_url(&self) -> Url {
        let organisation_name = &self.repository.project.organisation.name;
        let project_name = &self.repository.project.name;
        let repository_name = &self.repository.name;
        let pr_id = &self.id;

        let url_text = format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_git/{repository_name}/pullrequest/{pr_id}"
        );

        Url::from(url_text)
    }

    pub async fn get_linked_work_items(&self) -> Result<Vec<WorkItem>, CliError> {
        get_work_items(self).await
    }
}
