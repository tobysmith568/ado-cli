use std::{fs::File, path::PathBuf};

use dirs::home_dir;
use ini::Ini;

use crate::cli::cli_error::CliError;

pub struct ConfigFile {
    ini: Ini,
}

impl ConfigFile {
    pub fn load() -> Result<ConfigFile, CliError> {
        let full_path = resolve_config_path();

        ensure_file_exists(&full_path);

        let load_result = Ini::load_from_file(&full_path);

        match load_result {
            Ok(ini) => Ok(ConfigFile { ini }),
            Err(err) => CliError::err(&format!(
                "Cannot open config file {} because: {}",
                full_path.display(),
                err
            )),
        }
    }

    pub fn read_value_from_section(&self, section: &str, name: &str) -> Option<String> {
        self.ini
            .get_from(Some(section), name)
            .map(|value| value.to_string())
    }

    pub fn set_value_to_section(&mut self, section: &str, name: &str, value: &str) {
        self.ini.with_section(Some(section)).set(name, value);
    }

    pub fn save_config_to_disk(&self) {
        let full_path = resolve_config_path();

        _ = self.ini.write_to_file(full_path);
    }
}

fn resolve_config_path() -> PathBuf {
    home_dir()
        .expect("Cannot determine your home directory")
        .join(".ado_cli")
}

fn ensure_file_exists(path: &PathBuf) {
    if !path.exists() || !path.is_file() {
        _ = File::create(path);
    }
}
