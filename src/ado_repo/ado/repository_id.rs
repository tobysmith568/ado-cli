use std::{fs::read_to_string, fs::write, path::PathBuf};

use azure_devops_rust_api::git;

use crate::ado_repo::AdoRepo;

use super::api_key::get_api_key;

pub fn get_repository_id_from_disk(ado_repo: &AdoRepo) -> Option<String> {
    let repo_id_file_name = get_repo_id_file_name(&ado_repo);

    let file_content = read_to_string(repo_id_file_name);

    match file_content {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

pub fn save_repository_id_to_disk(ado_repo: &AdoRepo, repository_id: &str) {
    let repo_id_file_name = get_repo_id_file_name(&ado_repo);
    let write_result = write(repo_id_file_name, repository_id);

    match write_result {
        _ => (),
    }
}

pub async fn get_repository_id_from_api(ado_repo: &AdoRepo) -> String {
    let api_key = get_api_key();

    let AdoRepo {
        organisation_name,
        project_name,
        repository_name,
        ..
    } = &ado_repo;

    let git_client = git::ClientBuilder::new(api_key).build();

    let repo = git_client
        .repositories_client()
        .get_repository(organisation_name, repository_name, project_name)
        .await
        .unwrap();

    repo.id
}

fn get_repo_id_file_name(ado_repo: &AdoRepo) -> PathBuf {
    PathBuf::from(&ado_repo.directory).join("ado_repo_id")
}
