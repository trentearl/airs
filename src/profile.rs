use std::fs;
use std::process::Command;

use serde::Deserialize;
use serde::Serialize;
use tempfile::Builder;
use tracing::debug;

use crate::config;
use crate::files;
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

    #[serde(rename = "https://api.openai.com/v1/images/generations")]
    OpenAIImageGeneration,
}

impl Prof for openai_v1_chat::OpenAIChatCompletion {
    fn kind(&self) -> Kind {
        Kind::OpenAIV1ChatCompletion
    }
}

impl Prof for openai_v1_image::OpenAIImageGeneration {
    fn kind(&self) -> Kind {
        Kind::OpenAIV1ImageGeneration
    }
}

trait Prof {
    fn kind(&self) -> Kind;
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "kind")]
pub enum Profile {
    #[serde(rename = "https://api.openai.com/v1/chat/completions")]
    OpenAIV1ChatCompletion(openai_v1_chat::OpenAIChatCompletion),

    #[serde(rename = "https://api.openai.com/v1/images/generations")]
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

    let string = files::read_profile_file(&name);
    let profile: ProfileSimple = serde_json::from_str(&string).unwrap();
    if profile.kind == "https://api.openai.com/v1/chat/completions" {
        let p = openai_v1_chat::profile(&string);
        Profile::OpenAIV1ChatCompletion(p)
    } else if profile.kind == "https://api.openai.com/v1/images/generations" {
        let p = openai_v1_image::profile(&string);
        Profile::OpenAIV1ImageGeneration(p)
    } else {
        panic!("unknown profile kind");
    }
}

pub fn profile_current_text() -> String {
    config::read_config()
        .default_profile
        .unwrap_or("default".to_string())
}

fn get_prompt(messages: &[openai_v1_chat::OpenAIChatMessage]) -> String {
    let l = messages.len();

    if l == 0 {
        String::from("")
    } else {
        messages[l - 1].content.clone()
    }
}

fn edit(input: &String) -> String {
    let editor = std::env::var("EDITOR").unwrap_or("vi".to_string());
    let temp_file = Builder::new()
        .prefix("airs")
        .suffix(".json")
        .tempfile()
        .unwrap();

    let temp_path = temp_file
        .path()
        .to_str()
        .expect("Failed to get temp file path")
        .to_string();

    fs::write(&temp_path, input).expect("Failed to write to temporary file");

    Command::new(editor)
        .arg(&temp_path)
        .status()
        .expect("Failed to open editor");

    fs::read_to_string(temp_path).expect("Failed to read temporary file")
}

fn set_prompt(messages: &mut Vec<openai_v1_chat::OpenAIChatMessage>, content: String) {
    let l = messages.len();

    if l == 0 {
        messages.push(openai_v1_chat::OpenAIChatMessage {
            content,
            role: "user".to_string(),
        });
    } else {
        messages[l - 1].content = content;
    }
}

pub fn profile_edit(name_maybe: Option<String>) {
    let name = name_maybe.unwrap_or_else(profile_current_text);

    match profile_get(name.clone()) {
        Profile::OpenAIV1ChatCompletion(mut p) => {
            let prompt = edit(&get_prompt(&p.messages));
            set_prompt(&mut p.messages, prompt);
            let prof = Profile::OpenAIV1ChatCompletion(p);
            let content = serde_json::to_string_pretty(&prof).expect("failed to serialize profile");
            files::write_profile_file(&name, content);
        }

        Profile::OpenAIV1ImageGeneration(p) => {
            let prompt = edit(&p.prompt);
            let mut p = p;
            p.prompt = prompt;
            let prof = Profile::OpenAIV1ImageGeneration(p);
            let content = serde_json::to_string_pretty(&prof).expect("failed to serialize profile");
            files::write_profile_file(&name, content);
        }
    };
}

pub fn profile_edit_json(name: String) {
    let profiles = files::list_profiles();
    match profiles.iter().find(|&x| x == &name) {
        Some(_) => {}
        None => {
            panic!("profile: {} not found", name);
        }
    }

    let profile_string = files::read_profile_file(&name);
    let content = edit(&profile_string);
    let _: ProfileSimple = serde_json::from_str(&content).expect("Invalid JSON");

    files::write_profile_file(&name, content);
}

pub fn profile_new(name: String) {
    let current = config::read_config()
        .default_profile
        .unwrap_or("default".to_string());
    let profile = profile_get(current);

    let content = edit(&serde_json::to_string_pretty(&profile).unwrap());

    files::write_profile_file(&name, content);
}

pub fn profile_use(name: String) {
    let profiles = files::list_profiles();
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

    for profile in files::list_profiles() {
        if profile == current {
            println!("* {}", profile);
        } else {
            println!("  {}", profile);
        }
    }
}
