use std::env;

use reqwest::get;
use serde_json::json;
use tokio;

mod cli;
mod profile;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    match cli::parse() {
        cli::Cli {
            command: Some(cli::Commands::Profile { action }),
            freeform,
        } => {
            match action {
                cli::ProfileAction::New { name } => {
                    profile::profile_new(name);
                }
                cli::ProfileAction::Use { name } => {
                    profile::profile_use(name);
                }
                cli::ProfileAction::Remove { name } => {
                    profile::profile_remove(name);
                }
                cli::ProfileAction::List => {
                    profile::profile_list();
                }
            }
            println!("Freeform arguments: {:?}", freeform);
        }
        _ => {
            println!("No command provided");
        }
    }
    let openai_key = env::var("OPENAI_API_KEY").unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", openai_key))
        .json(&json!({
            "model": "gpt-4o",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant."
                },
                {
                    "role": "user",
                    "content": "What is the capital of china?"
                }
            ]
        }))
        .send()
        .await?;

    println!("{:#?}", res);
    let text = res.text().await?;
    println!("{}", text);

    Ok(())
}
