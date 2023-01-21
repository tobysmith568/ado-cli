mod resolve_storage_preference;
mod set_and_get_new_pat;
mod try_get_current_pat;

use crate::config::config_file::ConfigFile;

use self::{set_and_get_new_pat::set_and_get_new_pat, try_get_current_pat::try_get_current_pat};

pub fn get_api_key(config_file: &mut ConfigFile) -> String {
    let current_pat = try_get_current_pat(&config_file);

    if let Some(pat) = current_pat {
        return pat;
    }

    let new_pat = set_and_get_new_pat(config_file);

    new_pat
}
