use std::path::PathBuf;

use crate::ado::organisation::organisation::Organisation;

use super::repository::repository::Repository;

pub struct Project<'a> {
    pub name: String,
    pub organisation: &'a Organisation,
}

impl<'a> Project<'a> {
    pub fn new(name: &str, organisation: &'a Organisation) -> Project<'a> {
        Project {
            name: name.to_string(),
            organisation,
        }
    }

    pub fn get_repository(&self, name: &str, local_location: &PathBuf) -> Repository {
        Repository::new(self, name, local_location)
    }
}
