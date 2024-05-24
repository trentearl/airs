mod ai;
mod cli;
mod config;
mod files;
mod openai_v1_chat;
mod openai_v1_image;
mod profile;

use crate::profile::{profile_current_text, profile_get, Profile};
use is_terminal::IsTerminal;
use std::io::{self, Read};
use tracing::debug;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    tracing_subscriber::fmt::init();
    let args = cli::parse();

    match args.command {
        Some(cli::Commands::Profile { action }) => match action {
            cli::ProfileAction::New { name } => profile::profile_new(name),
            cli::ProfileAction::Use { name } => profile::profile_use(name),
            cli::ProfileAction::Remove { name } => profile::profile_remove(name),
            cli::ProfileAction::List => profile::profile_list(),
            cli::ProfileAction::Edit { name } => profile::profile_edit(name),
            cli::ProfileAction::EditJson { name } => profile::profile_edit_json(name),
        },
        None => {
            let profile_name = args.profile.unwrap_or_else(profile_current_text);
            debug!(profile_name, "found profile");
            let text = get_text(args.args);

            match profile_get(profile_name) {
                Profile::OpenAIV1ChatCompletion(chat) => {
                    debug!("profile kind: http://api.openai.com/v1/chat/completions");
                    ai::chat_completion(chat, text.join(" ")).await
                }
                Profile::OpenAIV1ImageGeneration(image) => {
                    debug!("profile kind: http://api.openai.com/v1/images/generations");
                    ai::image_generation(image, text.join(" ")).await
                }
            }
        }
    }
    return Ok(());
}

fn get_text(args: Vec<String>) -> Vec<String> {
    let mut text = vec![];
    for arg in args {
        text.push(arg);
    }

    if !io::stdin().is_terminal() {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        text.push(input);
    }

    text
}
