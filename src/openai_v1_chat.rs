use serde::Deserialize;
use serde::Serialize;

use crate::io;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct OpenAIChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OpenAIChatResponseFormatType {
    #[serde(rename = "json_object")]
    JsonObject,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct OpenAIChatCompletion {
    pub url: String,
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<OpenAIChatMessage>,
    pub response_format: Option<OpenAIChatResponseFormat>,
}

#[derive(Deserialize, Debug, Serialize)]
struct Intermediary {
    pub url: Option<String>,
    pub model: Option<String>,
    pub inherits: Option<Vec<String>>,
    pub max_tokens: Option<u32>,
    pub messages: Option<Vec<OpenAIChatMessage>>,
    pub json: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIChatResponseFormat {
    #[serde(rename = "type")]
    pub response_type: OpenAIChatResponseFormatType,
}

fn intermediate(s: &String) -> Vec<Intermediary> {
    let mut ret: Vec<Intermediary> = vec![];
    let base: Intermediary = serde_json::from_str(&s).unwrap();

    let iterable = match base.inherits.as_ref() {
        Some(i) => i,
        None => return vec![base],
    };

    for child in iterable {
        let mut i = intermediate(&io::read_profile_file(child));
        ret.append(&mut i);
    }

    ret.push(base);

    ret
}

pub fn default() -> OpenAIChatCompletion {
    return OpenAIChatCompletion {
        url: "https://api.openai.com/v1/chat/completions".to_string(),
        model: "gpt-4o".to_string(),
        max_tokens: 550,
        messages: vec![],
        response_format: None,
    };
}

pub fn profile(s: &String) -> OpenAIChatCompletion {
    let def = default();
    let intermediaries = intermediate(s);

    let mut url = def.url;
    let mut max_tokens = def.max_tokens;
    let mut messages: Vec<OpenAIChatMessage> = def.messages.clone();
    let mut model = def.model;
    let mut response_format = def.response_format;

    for i in intermediaries {
        if i.url.is_some() {
            url = i.url.unwrap();
        }
        if i.max_tokens.is_some() {
            max_tokens = i.max_tokens.unwrap();
        }
        if i.messages.is_some() {
            let mut msgs = i.messages.unwrap();
            messages.append(&mut msgs);
        }
        if i.model.is_some() {
            model = i.model.unwrap();
        }
        if i.json.is_some() {
            response_format = Some(OpenAIChatResponseFormat {
                response_type: OpenAIChatResponseFormatType::JsonObject,
            });
        }
    }

    return OpenAIChatCompletion {
        url,
        model,
        max_tokens,
        messages,
        response_format,
    };
}
