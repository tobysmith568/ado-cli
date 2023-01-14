use clap::{Parser, Subcommand};

use subcommands::{
    files::{run_files_command, Files},
    item::{run_item_command, Item},
    pr::{run_pr_command, Pr},
};

mod subcommands;
mod utils;

#[macro_use]
extern crate ini;

#[derive(clap::Parser, Debug)]
#[clap(
    author = "Toby Smith",
    version = "1.0.0",
    about = "Open Azure Devops board items, PRs, and repositories"
)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Item(Item),
    Pr(Pr),
    Files(Files),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Item(item) => run_item_command(item),
        Commands::Pr(pr) => run_pr_command(pr),
        Commands::Files(files) => run_files_command(files),
    }
}
