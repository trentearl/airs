use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(last = true)]
    pub freeform: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Profile {
        #[command(subcommand)]
        action: ProfileAction,
    },
}

#[derive(Subcommand)]
pub enum ProfileAction {
    New { name: String },
    Use { name: String },
    Remove { name: String },
    List,
}

pub fn parse() -> Cli {
    return Cli::parse();
}
