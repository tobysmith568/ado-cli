use std::{fs::read_to_string, fs::write, path::PathBuf};

use azure_devops_rust_api::git;

use crate::cli::cli_error::CliError;

use super::Repository;

pub async fn get_repository_id(repository: &Repository) -> Result<String, CliError> {
    let repository_id = get_repository_id_from_disk(repository);

    if let Some(id) = repository_id {
        return Ok(id);
    }

    let repository_id = get_repository_id_from_api(repository).await?;
    save_repository_id_to_disk(repository, &repository_id);

    Ok(repository_id)
}

fn get_repository_id_from_disk(repository: &Repository) -> Option<String> {
    let repo_id_file_name = get_repo_id_file_name(repository);

    let file_content = read_to_string(repo_id_file_name);

    match file_content {
        Ok(content) => Some(content),
        Err(_) => None,
    }
}

fn save_repository_id_to_disk(repository: &Repository, repository_id: &str) {
    let repo_id_file_name = get_repo_id_file_name(repository);
    let _write_result = write(repo_id_file_name, repository_id);
}

async fn get_repository_id_from_api(repository: &Repository) -> Result<String, CliError> {
    let credential = repository.project.organisation.create_credential();
    let git_client = git::ClientBuilder::new(credential).build();

    let organisation_name = &repository.project.organisation.name;
    let project_name = &repository.project.name;
    let repository_name = &repository.name;

    let repository_result = git_client
        .repositories_client()
        .get_repository(organisation_name, repository_name, project_name)
        .await;

    match repository_result {
        Ok(repo) => Ok(repo.id),
        Err(_) => {
            let message = "Invalid response from Azure Devops API.".to_string();
            CliError::err(&message)
        }
    }
}

fn get_repo_id_file_name(repository: &Repository) -> PathBuf {
    repository.local_location.join(".git").join("ado_repo_id")
}
