use std::env;

use crate::{cli::prompt_string::prompt_for_string, config::config_file::ConfigFile};

use super::resolve_storage_preference::{prompt_for_storage_preference, ApiKeyStorageChoice};

pub fn set_and_get_new_pat(config_file: &mut ConfigFile) -> String {
    let preference = prompt_for_storage_preference();

    let new_pat = match preference {
        ApiKeyStorageChoice::Pat => {
            let pat = prompt_for_string("Please paste in your ADO PAT:");
            config_file.set_value_to_section("credential", "pat", &pat);

            pat
        }
        ApiKeyStorageChoice::EnvVar => {
            let env_var = prompt_for_string("Please enter the name of your Environment Variable");

            let env_var_value = match env::var(&env_var) {
                Ok(value) => value,
                Err(err) => panic!("{}", &err),
            };

            if !env_var_value.is_empty() {
                config_file.set_value_to_section("credential", "env_var", &env_var);
            }

            env_var_value
        }
    };

    config_file.save_config_to_disk();

    new_pat
}
