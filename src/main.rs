use clap::{Parser, Subcommand};

use subcommands::{
    files::{run_files_command, Files},
    item::{run_item_command, Item},
    pr::{run_pr_command, Pr},
};

mod ado;
mod cli;
mod subcommands;
mod utils;

#[macro_use]
extern crate ini;

#[derive(clap::Parser, Debug)]
#[clap(
    author,
    version,
    about = "Open Azure Devops board items, PRs, and repositories"
)]
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

    /// Opens a browser tab to the PBI/Bug/Action/etc. that's associated with the PR for the current branch
    Item(Item),
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Files(files) => run_files_command(files),
        Commands::Pr(pr) => run_pr_command(pr).await,
        Commands::Item(item) => run_item_command(item).await,
    }
}
