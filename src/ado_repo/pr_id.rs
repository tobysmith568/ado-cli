use azure_devops_rust_api::git;

use crate::ado_repo::{ado::api_key::get_api_key, AdoRepo};

pub async fn get_pr_id(ado_repo: &AdoRepo) -> Option<i32> {
    let repo_id = ado_repo.get_repository_id().await;
    let source_branch = ado_repo.current_branch();

    let api_key = get_api_key();

    let AdoRepo {
        organisation_name,
        project_name,
        ..
    } = &ado_repo;

    let git_client = git::ClientBuilder::new(api_key).build();

    let pull_requests = git_client
        .pull_requests_client()
        .get_pull_requests(organisation_name, repo_id, project_name)
        .await
        .unwrap();

    for pr in pull_requests.value.iter() {
        if pr.source_ref_name.ends_with(&source_branch) {
            return Some(pr.pull_request_id);
        }
    }

    None
}
