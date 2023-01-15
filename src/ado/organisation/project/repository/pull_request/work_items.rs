use azure_devops_rust_api::git;

use crate::ado::{api_key::get_api_key, organisation::project::work_item::WorkItem};

use super::PullRequest;

pub async fn get_work_items<'a>(pull_request: &PullRequest<'a>) -> Vec<WorkItem<'a>> {
    let api_key = get_api_key();

    let organisation_name = &pull_request.repository.project.organisation.name;
    let project_name = &pull_request.repository.project.name;
    let repository_name = &pull_request.repository.name;
    let pr_id = pull_request.id;

    let git_client = git::ClientBuilder::new(api_key).build();

    let work_items = git_client
        .pull_request_work_items_client()
        .list(organisation_name, repository_name, pr_id, project_name)
        .await
        .unwrap()
        .value;

    work_items
        .into_iter()
        .map(|wi| WorkItem::new(pull_request.repository.project, &wi.id.unwrap()))
        .collect::<Vec<WorkItem>>()
}
