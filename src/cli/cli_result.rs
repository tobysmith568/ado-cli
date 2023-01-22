use super::cli_error::CliError;

pub enum CliResult {
    Success,
    Err(CliError),
}
