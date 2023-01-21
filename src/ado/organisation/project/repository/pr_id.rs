use azure_devops_rust_api::git;

use super::{repository_id::get_repository_id, Repository};

pub async fn get_pr_id<'a>(repository: &Repository<'a>, branch_name: &str) -> Option<i32> {
    let credential = repository.project.organisation.create_credential();
    let git_client = git::ClientBuilder::new(credential).build();

    let organisation_name = &repository.project.organisation.name;
    let project_name = &repository.project.name;
    let repo_id = get_repository_id(repository).await;

    let pull_requests = git_client
        .pull_requests_client()
        .get_pull_requests(organisation_name, repo_id, project_name)
        .await
        .unwrap();

    for pr in pull_requests.value.iter() {
        if pr.source_ref_name.ends_with(branch_name) {
            return Some(pr.pull_request_id);
        }
    }

    None
}
