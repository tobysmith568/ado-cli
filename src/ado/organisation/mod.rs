use self::project::Project;

pub mod project;

pub struct Organisation {
    pub name: String,
}

impl Organisation {
    pub fn new(name: &str) -> Organisation {
        Organisation {
            name: name.to_string(),
        }
    }

    pub fn get_project(&self, name: &str) -> Project {
        Project::new(name, self)
    }
}
