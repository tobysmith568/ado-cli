use self::pr_work_items::get_pr_work_items;

use super::{ado_work_item::AdoWorkItem, AdoRepo};

mod pr_work_items;

pub struct AdoPr<'a> {
    ado_repo: &'a AdoRepo,
    id: i32,
}

impl<'a> AdoPr<'a> {
    pub fn from(ado_repo: &'a AdoRepo, id: i32) -> AdoPr {
        AdoPr { ado_repo, id }
    }

    pub async fn pr_page_url(&self) -> Option<String> {
        let pr_id = self.id;

        let AdoRepo {
            organisation_name,
            project_name,
            repository_name,
            ..
        } = self.ado_repo;

        Some(format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_git/{repository_name}/pullrequest/{pr_id}"
        ))
    }

    pub async fn pr_work_items(&self) -> Vec<AdoWorkItem> {
        get_pr_work_items(&self.ado_repo, self.id).await
    }
}
