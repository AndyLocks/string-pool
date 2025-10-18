use crate::commands::add::add;
use crate::commands::build::build;
use crate::commands::commands::Commands;
use crate::commands::get::get;
use crate::commands::list::list;
use crate::commands::remove::remove;
use clap::CommandFactory;
use clap::Parser;
use std::process::exit;

mod commands;
mod config;

#[derive(Parser)]
#[command(
    name = "string-pool",
    version,
    about = "String Pool",
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
                    println!("IO Error: {x}");
                    exit(1);
                })
                .ok();
        }
        Commands::Add { dir, key } => {
            add(dir, key.as_str())
                .inspect_err(|x| {
                    println!("IO Error: {x}");
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
                    println!("IO Error: {x}");
                    exit(1);
                })
                .ok();
        }
        Commands::Remove { dir, key } => {
            remove(dir, key.as_str())
                .inspect_err(|x| {
                    println!("IO Error: {x}");
                    exit(1);
                })
                .ok();
        }
        Commands::Build { key, value } => build(key, value),
    }
}
