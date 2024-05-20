use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long)]
    pub profile: Option<String>,

    #[arg(trailing_var_arg = true)]
    pub args: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(alias = "prof")]
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
}

#[derive(Subcommand)]
pub enum EditField {
    Json { name: String },
    Prompt { name: String },
}

#[derive(Subcommand)]
pub enum ProfileAction {
    New {
        name: String,
    },
    Edit {
        name: String,
    },
    EditJson {
        name: String,
    },
    Use {
        name: String,
    },
    Remove {
        name: String,
    },

    #[clap(alias = "ls")]
    List,
}

pub fn parse() -> Cli {
    Cli::parse()
}
