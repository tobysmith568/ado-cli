use std::env;

use crate::{
    cli::{cli_error::CliError, prompt_string::prompt_for_string},
    config::config_file::ConfigFile,
};

use super::resolve_storage_preference::{prompt_for_storage_preference, ApiKeyStorageChoice};

pub fn set_and_get_new_pat(config_file: &mut ConfigFile) -> Result<String, CliError> {
    let preference = prompt_for_storage_preference();

    let new_pat = match preference {
        ApiKeyStorageChoice::Pat => get_new_pat(config_file),
        ApiKeyStorageChoice::EnvVar => get_new_env_var(config_file)?,
    };

    config_file.save_config_to_disk();

    Ok(new_pat)
}

fn get_new_pat(config_file: &mut ConfigFile) -> String {
    let pat = prompt_for_string("Please paste in your ADO PAT:");

    config_file.set_value_to_section("credential", "pat", &pat);

    pat
}

fn get_new_env_var(config_file: &mut ConfigFile) -> Result<String, CliError> {
    let env_var = prompt_for_string("Please enter the name of your Environment Variable");

    let env_var_value = match env::var(&env_var) {
        Ok(value) => value,
        Err(err) => return CliError::err(&format!("{}", err)),
    };

    if !env_var_value.is_empty() {
        config_file.set_value_to_section("credential", "env_var", &env_var);
    }

    Ok(env_var_value)
}
