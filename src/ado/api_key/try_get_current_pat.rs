use std::env;

use crate::config::config_file::ConfigFile;

pub fn try_get_current_pat(config_file: &ConfigFile) -> Option<String> {
    let pat = config_file.read_value_from_section("credential", "pat");

    if pat.is_some() {
        return pat;
    }

    let env_var = config_file.read_value_from_section("credential", "env_var");

    if let Some(env_var) = env_var {
        let env_var_value = env::var(env_var);

        if let Ok(value) = env_var_value {
            return Some(value);
        }
    }

    None
}
