use crate::commands::add::add;
use crate::commands::commands::Commands;
use crate::commands::edit::edit;
use crate::commands::get::get;
use crate::commands::key::key;
use crate::commands::list::list;
use crate::commands::remove::remove;
use clap::CommandFactory;
use clap::Parser;
use std::process::exit;

mod commands;

#[derive(Parser)]
#[command(
    name = "string-pool",
    version,
    about,
    after_help = "Copyright (C) 2026  Illia <jandylokc@gmail.com>\nLicense GPL-3.0-or-later",
    arg_required_else_help = false
)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

fn main() {
    match Cli::parse().command {
        Commands::Get { dir, key, s } => {
            get(dir, key.as_str(), s)
                .inspect_err(|x| {
                    eprintln!("IO Error: {x}");
                    exit(1);
                })
                .ok();
        }
        Commands::Add { dir, key } => {
            add(dir, key.as_str())
                .inspect_err(|x| {
                    eprintln!("IO Error: {x}");
                    exit(1);
                })
                .ok();
        }
        Commands::Completions { shell } => clap_complete::generate(
            shell,
            &mut Cli::command(),
            "stringp",
            &mut std::io::stdout(),
        ),
        Commands::List { dir } => {
            list(dir)
                .inspect_err(|x| {
                    eprintln!("IO Error: {x}");
                    exit(1);
                })
                .ok();
        }
        Commands::Remove { dir, key } => {
            remove(dir, key.as_str())
                .inspect_err(|x| {
                    eprintln!("IO Error: {x}");
                    exit(1);
                })
                .ok();
        }
        Commands::Key { argument, value } => key(argument, value),
        Commands::Edit { dir, key } => {
            if let Err(message) = edit(dir, &key) {
                eprintln!("{message}");
                exit(1)
            }
        }
    }
}
