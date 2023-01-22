use std::process::ExitCode;

use ado::api_key::get_api_key;
use clap::{Parser, Subcommand};

use cli::cli_result::CliResult;
use config::config_file::ConfigFile;
use subcommands::{
    files::{run_files_command, Files},
    item::{run_item_command, Item},
    pr::{run_pr_command, Pr},
};

mod ado;
mod cli;
mod config;
mod subcommands;
mod utils;

extern crate ini;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Opens a browser tab to the file-browser page for the current branch
    Files(Files),

    /// Opens a browser tab to the open PR for the current branch, or lets you create one
    Pr(Pr),

    /// [Aliases: pbi & bug] Opens a browser tab to the PBI/Bug/Action/etc. that's associated with the PR for the current branch
    #[clap(aliases = ["pbi", "bug"])]
    Item(Item),
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli_result = implementation().await;

    if let CliResult::Err(err) = cli_result {
        println!("{}", err);
        return ExitCode::from(1);
    }

    ExitCode::from(0)
}

async fn implementation() -> CliResult {
    let cli = Cli::parse();

    let mut config_file = match ConfigFile::load() {
        Ok(file) => file,
        Err(err) => return err.into_result(),
    };

    let api_key = match get_api_key(&mut config_file) {
        Ok(api_key) => api_key,
        Err(err) => return err.into_result(),
    };

    match cli.command {
        Commands::Files(files) => run_files_command(files, api_key),
        Commands::Pr(pr) => run_pr_command(pr, api_key).await,
        Commands::Item(item) => run_item_command(item, api_key).await,
    }
}
