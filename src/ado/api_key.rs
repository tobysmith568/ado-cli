use std::fs::read_to_string;

use azure_devops_rust_api::Credential;
use dirs::home_dir;

pub fn get_api_key() -> Credential {
    let full_path = home_dir().unwrap().join(".ado_cli");

    let value = read_to_string(full_path).unwrap();
    Credential::Pat(value)
}
