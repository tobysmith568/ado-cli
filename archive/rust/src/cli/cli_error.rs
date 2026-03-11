use std::fmt::Display;

use super::cli_result::CliResult;

pub struct CliError {
    message: String,
}

impl CliError {
    pub fn new(message: &str) -> CliError {
        let message = String::from(message);

        CliError { message }
    }

    pub fn err<T>(message: &str) -> std::result::Result<T, CliError> {
        Err(CliError::new(message))
    }

    pub fn into_result(self) -> CliResult {
        CliResult::Err(self)
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
