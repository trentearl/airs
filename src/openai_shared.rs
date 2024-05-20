use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Deserialize, Debug, Serialize)]
pub struct OpenAIChatMessage {
    pub role: String,
    pub content: String,
}
