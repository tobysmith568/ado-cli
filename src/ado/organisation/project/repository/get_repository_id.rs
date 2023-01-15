use std::{fs::read_to_string, fs::write, path::PathBuf};

use azure_devops_rust_api::git;

use crate::ado::api_key::get_api_key;

use super::repository::Repository;

pub async fn get_repository_id<'a>(repository: &Repository<'a>) -> String {
    let repository_id = get_repository_id_from_disk(&repository);

    if let Some(id) = repository_id {
        return id;
    }

    let repository_id = get_repository_id_from_api(&repository).await;
    save_repository_id_to_disk(&repository, &repository_id);
    repository_id
}

fn get_repository_id_from_disk<'a>(repository: &Repository<'a>) -> Option<String> {
    let repo_id_file_name = get_repo_id_file_name(repository);

    let file_content = read_to_string(repo_id_file_name);

    match file_content {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

fn save_repository_id_to_disk<'a>(repository: &Repository<'a>, repository_id: &str) {
    let repo_id_file_name = get_repo_id_file_name(repository);
    let write_result = write(repo_id_file_name, repository_id);

    match write_result {
        _ => (),
    }
}

async fn get_repository_id_from_api<'a>(repository: &Repository<'a>) -> String {
    let api_key = get_api_key();

    let organisation_name = &repository.project.organisation.name;
    let project_name = &repository.project.name;
    let repository_name = &repository.name;

    let git_client = git::ClientBuilder::new(api_key).build();

    let repo = git_client
        .repositories_client()
        .get_repository(organisation_name, repository_name, project_name)
        .await
        .unwrap();

    repo.id
}

fn get_repo_id_file_name<'a>(repository: &Repository<'a>) -> PathBuf {
    repository.local_location.join("ado_repo_id")
}
