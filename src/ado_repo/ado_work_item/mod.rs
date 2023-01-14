pub struct AdoWorkItem {
    organisation_name: String,
    project_name: String,
    id: String,
}

impl AdoWorkItem {
    pub fn new(organisation_name: String, project_name: String, id: String) -> AdoWorkItem {
        AdoWorkItem {
            organisation_name,
            project_name,
            id,
        }
    }

    pub fn get_url(&self) -> String {
        let AdoWorkItem {
            organisation_name,
            project_name,
            id: work_item_id,
            ..
        } = self;

        format!(
            "https://dev.azure.com/{organisation_name}/{project_name}/_workitems/edit/{work_item_id}"
        )
    }
}
