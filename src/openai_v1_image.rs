use serde::Deserialize;
use serde::Serialize;

use crate::io;

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIImageGeneration {
    pub url: String,
    pub model: String,
    pub n: u8,
    pub size: OpenAIImageSize,
    pub prompt: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OpenAIImageSize {
    #[serde(rename = "256x256")]
    Size256x256,

    #[serde(rename = "512x512")]
    Size512x512,

    #[serde(rename = "1024x1024")]
    Size1024x1024,

    #[serde(rename = "1280x720")]
    Size1792x1024,

    #[serde(rename = "2560x1440")]
    Size1024x1792,
}

pub fn default() -> OpenAIImageGeneration {
    OpenAIImageGeneration {
        url: "https://api.openai.com/v1/images/generations".to_string(),
        model: "dall-e-3".to_string(),
        n: 1,
        size: OpenAIImageSize::Size1024x1024,
        prompt: "".to_string(),
    }
}

#[derive(Deserialize, Debug, Serialize)]
struct Intermediary {
    pub url: Option<String>,
    pub model: Option<String>,
    pub inherits: Option<Vec<String>>,
    pub n: Option<u8>,
    pub prompt: Option<String>,
    pub size: Option<OpenAIImageSize>,
}

fn intermediate(s: &str) -> Vec<Intermediary> {
    let mut ret: Vec<Intermediary> = vec![];
    let base: Intermediary = serde_json::from_str(s).unwrap();

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

pub fn profile(s: &str) -> OpenAIImageGeneration {
    let def = default();
    let intermediaries = intermediate(s);

    let mut url = def.url;
    let mut n = def.n;
    let mut prompt = def.prompt;
    let mut model = def.model;
    let mut size = def.size;

    for i in intermediaries {
        if i.url.is_some() {
            url = i.url.unwrap();
        }

        if i.n.is_some() {
            n = i.n.unwrap();
        }

        if i.prompt.is_some() {
            prompt = i.prompt.unwrap();
        }

        if i.model.is_some() {
            model = i.model.unwrap();
        }

        if i.size.is_some() {
            size = i.size.unwrap();
        }
    }

    OpenAIImageGeneration {
        url,
        model,
        n,
        prompt,
        size,
    }
}
