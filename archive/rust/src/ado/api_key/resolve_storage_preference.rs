use std::fmt::{Display, Formatter, Result};

use crate::cli::prompt_enum::prompt_enum;

pub enum ApiKeyStorageChoice {
    Pat,
    EnvVar,
}

impl Display for ApiKeyStorageChoice {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ApiKeyStorageChoice::Pat => write!(f, "Paste in a PAT directly"),
            ApiKeyStorageChoice::EnvVar => write!(f, "Given an Environment Variable name"),
        }
    }
}

pub fn prompt_for_storage_preference() -> ApiKeyStorageChoice {
    println!("This CLI tool requires an ADO PAT in order to function.");
    println!("You can either paste an APO PAT directly into this CLI, or supply the name of an Environment Variable where you will manually store an ADO PAT");
    println!();

    let options = vec![ApiKeyStorageChoice::Pat, ApiKeyStorageChoice::EnvVar];

    let index = prompt_enum("Which method would you prefer?", &options).unwrap();

    match index {
        0 => ApiKeyStorageChoice::Pat,
        _ => ApiKeyStorageChoice::EnvVar,
    }
}
