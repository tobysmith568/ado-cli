use std::fmt::{Debug, Display, Formatter, Result};

use super::prompt_enum::prompt_enum;

#[derive(Debug)]
pub enum YesNoResult {
    Yes,
    No,
}

impl Display for YesNoResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Debug::fmt(&self, f)
    }
}

pub fn prompt_yes_no(question: &str) -> YesNoResult {
    let options = vec![YesNoResult::Yes, YesNoResult::No];

    let index = prompt_enum(question, &options).unwrap_or(1);

    match index {
        0 => YesNoResult::Yes,
        _ => YesNoResult::No,
    }
}
