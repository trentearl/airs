use serde::Deserialize;
use serde_json::json;
use std::env;
use tracing::debug;

use crate::openai_shared::OpenAIChatMessage;
use crate::openai_v1_chat::OpenAIChatCompletion;
use crate::openai_v1_image::OpenAIImageGeneration;

pub async fn chat_completion(profile: OpenAIChatCompletion, args: String) {
    let openai_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");
    debug!("found OPENAI_API_KEY");

    let client = reqwest::Client::new();
    let user_message = OpenAIChatMessage {
        role: "user".to_string(),
        content: args,
    };
    let mut messages = profile.messages;
    messages.push(user_message);
    debug!("messages: {:?}", messages);
    let url = profile.url;

    debug!(url, profile.model, profile.max_tokens, "begin request");
    let payload = json!({
        "model": profile.model,
        "messages": messages,
        "response_format": profile.response_format,
        "max_tokens": profile.max_tokens
    });

    debug!("payload: {:?}", payload);

    let res = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", openai_key))
        .json(&payload)
        .send()
        .await
        .expect("openai error");
    let status = res.status();
    if !status.is_success() {
        let t = res.json::<OpenAIErrorResponse>().await.expect("weird json");
        panic!("openai error: {}", t.error.message);
    }

    let json = res.json::<ChatCompletion>().await.expect("weird json");
    debug!(
        json.id,
        json.object, json.created, json.model, "response metadata"
    );
    debug!(
        json.usage.prompt_tokens,
        json.usage.completion_tokens, json.usage.total_tokens, "usage"
    );
    for choice in json.choices {
        debug!(
            choice.index,
            choice.finish_reason, choice.message.role, "choice"
        );
        println!("{}", choice.message.content);
    }
}

pub async fn image_generation(profile: OpenAIImageGeneration, args: String) {
    let openai_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY is not set");

    let client = reqwest::Client::new();

    let payload = json!({
        "model": profile.model,
        "prompt": args,
        "n": profile.n,
        "size": profile.size,
    });

    let res = client
        .post(&profile.url)
        .header("Authorization", format!("Bearer {}", openai_key))
        .json(&payload)
        .send()
        .await
        .expect("openai error");

    let status = res.status();
    if !status.is_success() {
        let t = res.json::<OpenAIErrorResponse>().await.expect("weird json");
        panic!("openai error: {}", t.error.message);
    }

    let json = res
        .json::<ImageGenerationResponse>()
        .await
        .expect("weird json");

    for image in json.data {
        println!("{}", image.url);
    }
}

#[derive(Deserialize, Debug)]
struct OpenAIErrorResponse {
    error: OpenAIError,
}

#[derive(Deserialize, Debug)]
struct OpenAIError {
    message: String,
}

#[derive(Deserialize, Debug)]
struct ImageGeneration {
    pub url: String,
}

#[derive(Deserialize, Debug)]
struct ImageGenerationResponse {
    pub data: Vec<ImageGeneration>,
}

#[derive(Deserialize, Debug)]
struct ChatCompletion {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Deserialize, Debug)]
struct Choice {
    index: u8,
    message: Message,
    finish_reason: String,
}

#[derive(Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize, Debug)]
struct Usage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}
