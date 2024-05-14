use serde::Deserialize;
use tracing::debug;

use crate::config;
use crate::io;
use crate::openai_v1_chat;
use crate::openai_v1_image;

pub enum Kind {
    OpenAIV1ChatCompletion,
    OpenAIV1ImageGeneration,
}

// Define an enum to represent different kinds
#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
enum ProfKind {
    #[serde(rename = "https://api.openai.com/v1/chat/completions")]
    OpenAIChatCompletion,

    #[serde(rename = "https://api.openai.com/v1/image/generate")]
    OpenAIImageGeneration,
}

impl Prof for openai_v1_chat::OpenAIChatCompletion {
    fn kind(&self) -> Kind {
        Kind::OpenAIV1ChatCompletion
    }
}

impl Prof for openai_v1_image::OpenAIImageGeneration {
    fn kind(&self) -> Kind {
        Kind::OpenAIV1ChatCompletion
    }
}

trait Prof {
    fn kind(&self) -> Kind;
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Profile {
    #[serde(rename = "https://api.openai.com/v1/chat/completions")]
    OpenAIV1ChatCompletion(openai_v1_chat::OpenAIChatCompletion),

    #[serde(rename = "https://api.openai.com/v1/image/generate")]
    OpenAIV1ImageGeneration(openai_v1_image::OpenAIImageGeneration),
}

#[derive(Deserialize)]
struct ProfileSimple {
    kind: String,
}

pub fn profile_get(name: String) -> Profile {
    if name == "default" {
        let def = openai_v1_chat::default();
        return Profile::OpenAIV1ChatCompletion(def);
    }

    let string = io::read_profile_file(&name);
    let profile: ProfileSimple = serde_json::from_str(&string).unwrap();
    if profile.kind == "https://api.openai.com/v1/chat/completions" {
        let p = openai_v1_chat::profile(&string);
        return Profile::OpenAIV1ChatCompletion(p);
    } else if profile.kind == "https://api.openai.com/v1/image/generate" {
        return Profile::OpenAIV1ImageGeneration(
            serde_json::from_str(&string).expect("failed to parse profile"),
        );
    } else {
        panic!("unknown profile kind");
    }
}

pub fn profile_current_text() -> String {
    config::read_config()
        .default_profile
        .unwrap_or("default".to_string())
}

pub fn profile_new(name: String) {
    debug!(name, "profile_new");
}

pub fn profile_use(name: String) {
    let profiles = io::list_profiles();
    match profiles.iter().find(|&x| x == &name) {
        Some(_) => {}
        None => {
            panic!("profile: {} not found", name);
        }
    }
    config::set_default_profile(name);
}

pub fn profile_remove(name: String) {
    debug!(name, "profile_remove");
}

pub fn profile_list() {
    let current = config::read_config()
        .default_profile
        .unwrap_or("default".to_string());

    for profile in io::list_profiles() {
        if profile == current {
            println!("* {}", profile);
        } else {
            println!("  {}", profile);
        }
    }
}
