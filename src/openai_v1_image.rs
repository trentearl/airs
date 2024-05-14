use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct OpenAIImageMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenAIImageGeneration {
    pub inherits: Option<Vec<String>>,
    model: String,
    n: u32,
    size: OpenAIImageSize,
    messages: Vec<OpenAIImageMessage>,
}

#[derive(Debug, Deserialize, Serialize)]
enum OpenAIImageSize {
    Size256x256,
    Size512x512,
    Size1024x1024,
    Size1792x1024,
    Size1024x1792,
}
