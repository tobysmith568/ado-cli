use crate::{ado::organisation::project::project::Project, utils::url::Url};

pub struct WorkItem<'a> {
    id: String,
    project: &'a Project<'a>,
}

impl<'a> WorkItem<'a> {
    pub fn new(project: &'a Project, id: &str) -> WorkItem<'a> {
        WorkItem {
            id: id.to_string(),
            project,
        }
    }

    pub fn get_url(&self) -> Url {
        let organisation_name = &self.project.organisation.name;
        let project_name = &self.project.name;
        let work_item_id = &self.id;

        let url_text = format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_workitems/edit/{work_item_id}"
        );

        Url::from(url_text)
    }
}
