use azure_devops_rust_api::Credential;

use self::project::Project;

pub mod project;

pub struct Organisation {
    pub name: String,
    api_key: String,
}

impl Organisation {
    pub fn new(name: &str, api_key: String) -> Organisation {
        Organisation {
            name: name.to_string(),
            api_key,
        }
    }

    pub fn get_project(&self, name: &str) -> Project {
        Project::new(name, self)
    }

    fn create_credential(&self) -> Credential {
        Credential::from_pat(&self.api_key)
    }
}
