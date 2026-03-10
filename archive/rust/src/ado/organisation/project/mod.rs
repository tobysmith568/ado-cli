use std::path::Path;

use self::{repository::Repository, work_item::WorkItem};

use super::Organisation;

pub mod repository;
pub mod work_item;

pub struct Project {
    pub name: String,
    pub organisation: Organisation,
}

impl Project {
    pub fn new(name: &str, organisation: Organisation) -> Project {
        Project {
            name: name.to_string(),
            organisation,
        }
    }

    pub fn get_repository(self, name: &str, local_location: &Path) -> Repository {
        Repository::new(self, name, local_location)
    }

    pub fn get_work_item(&self, id: String) -> WorkItem {
        WorkItem::new(self, &id)
    }
}
