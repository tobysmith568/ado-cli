use azure_devops_rust_api::git;

use crate::cli::cli_error::CliError;

use super::{repository_id::get_repository_id, Repository};

pub async fn get_pr_id(
    repository: &Repository,
    branch_name: &str,
) -> Result<Option<i32>, CliError> {
    let credential = repository.project.organisation.create_credential();
    let git_client = git::ClientBuilder::new(credential).build();

    let organisation_name = &repository.project.organisation.name;
    let project_name = &repository.project.name;
    let repo_id = get_repository_id(repository).await?;

    let pull_requests_result = git_client
        .pull_requests_client()
        .get_pull_requests(organisation_name, repo_id, project_name)
        .await;

    let pull_requests = match pull_requests_result {
        Ok(prs) => prs,
        Err(_) => {
            let message = "Invalid response from Azure Devops API.".to_string();
            return CliError::err(&message);
        }
    };

    for pr in pull_requests.value.iter() {
        if pr.source_ref_name.ends_with(branch_name) {
            return Ok(Some(pr.pull_request_id));
        }
    }

    Ok(None)
}
