use tokio;
use tracing::debug;
use tracing_subscriber;

mod ai;
mod cli;
mod config;
mod io;
mod openai_v1_chat;
mod openai_v1_image;
mod profile;

use crate::profile::{profile_current_text, profile_get, Profile};

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
        },
        None => {
            let profile_name = args.profile.unwrap_or_else(|| profile_current_text());
            debug!(profile_name, "found profile");
            let text = args.args;

            match profile_get(profile_name) {
                Profile::OpenAIV1ChatCompletion(chat) => {
                    debug!("profile kind: http://api.openai.com/v1/chat/completions");
                    ai::chat_completion(chat, text.join(" ")).await
                }
                _ => {
                    panic!("i cant handle this profile type")
                }
            }
        }
    }
    return Ok(());
}
