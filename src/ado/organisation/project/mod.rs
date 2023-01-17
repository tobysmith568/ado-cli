use std::path::Path;

use self::{repository::Repository, work_item::WorkItem};

use super::Organisation;

pub mod repository;
pub mod work_item;

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

    pub fn get_repository(&self, name: &str, local_location: &Path) -> Repository {
        Repository::new(self, name, local_location)
    }

    pub fn get_work_item(&self, id: String) -> WorkItem {
        WorkItem::new(self, &id)
    }
}
