use azure_devops_rust_api::git;

use crate::ado_repo::{ado::api_key::get_api_key, ado_work_item::AdoWorkItem, AdoRepo};

pub async fn get_pr_work_items(ado_repo: &AdoRepo, pr_id: i32) -> Vec<AdoWorkItem> {
    let api_key = get_api_key();

    let AdoRepo {
        organisation_name,
        project_name,
        repository_name,
        ..
    } = &ado_repo;

    let git_client = git::ClientBuilder::new(api_key).build();

    let work_items = git_client
        .pull_request_work_items_client()
        .list(organisation_name, repository_name, pr_id, project_name)
        .await
        .unwrap()
        .value;

    work_items
        .into_iter()
        .map(|wi| {
            AdoWorkItem::new(
                organisation_name.to_string(),
                project_name.to_string(),
                wi.id.unwrap(),
            )
        })
        .collect::<Vec<AdoWorkItem>>()
}
