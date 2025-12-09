use std::path::PathBuf;
use clap::Subcommand;
use clap_complete::Shell;

#[derive(Subcommand)]
pub enum Commands {
    #[command(
        about = "Outputs content by key",
        alias = "g",
        help_expected = true
    )]
    Get {
        #[arg(help = "Filename in the String Pool directory")]
        key: String,

        #[arg(short, long, help = "String Pool directory")]
        dir: Option<PathBuf>,

        #[arg(short, long, help = "Enable safe format mode", default_value = "false")]
        s: bool,
    },

    #[command(
        about = "Write stdin to file in the String Pool directory named by key",
        alias = "a",
        help_expected = true
    )]
    Add {
        #[arg(help = "File name")]
        key: String,

        #[arg(short, long, help = "String Pool directory")]
        dir: Option<PathBuf>,
    },

    #[command(
        about = "Remove file in the String Pool directory by key",
        alias = "rm",
        help_expected = true
    )]
    Remove {
        #[arg(help = "File name")]
        key: String,

        #[arg(short, long, help = "String Pool directory")]
        dir: Option<PathBuf>,
    },

    #[command(
        about = "Outputs all available keys",
        aliases = ["l", "keys"],
        help_expected = true,
    )]
    List {
        #[arg(short, long, help = "String Pool directory")]
        dir: Option<PathBuf>,
    },

    #[command(
        about = "Arguments builder",
        alias = "k",
        help_expected = true,
    )]
    Key {
        #[arg()]
        argument: String,

        #[arg()]
        value: String,
    },

    #[command(about = "Generate auto completion", help_expected = true)]
    Completions {
        #[arg(help = "Your shell name (zsh, bash, fish, elvish, powershell)")]
        shell: Shell,
    },
}