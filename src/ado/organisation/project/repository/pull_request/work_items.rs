use azure_devops_rust_api::git;

use crate::{ado::organisation::project::work_item::WorkItem, cli::cli_error::CliError};

use super::PullRequest;

pub async fn get_work_items<'a>(
    pull_request: &PullRequest<'a>,
) -> Result<Vec<WorkItem<'a>>, CliError> {
    let credential = pull_request
        .repository
        .project
        .organisation
        .create_credential();
    let git_client = git::ClientBuilder::new(credential).build();

    let organisation_name = &pull_request.repository.project.organisation.name;
    let project_name = &pull_request.repository.project.name;
    let repository_name = &pull_request.repository.name;
    let pr_id = pull_request.id;

    let work_items_result = git_client
        .pull_request_work_items_client()
        .list(organisation_name, repository_name, pr_id, project_name)
        .await;

    let work_items = match work_items_result {
        Ok(wi) => wi.value,
        Err(_) => {
            let message = "Invalid response from Azure Devops API.".to_string();
            return CliError::err(&message);
        }
    };

    Ok(work_items
        .into_iter()
        .map(|wi| WorkItem::new(&pull_request.repository.project, &wi.id.unwrap()))
        .collect::<Vec<WorkItem>>())
}
