use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub enum Model {
    #[serde(rename = "gpt-3.5-turbo-16k")]
    Gpt3_5Turbo16k,
}

#[derive(Serialize)]
pub struct OpenAiRequest {
    pub model: Model,
    pub messages: Vec<Message>,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct OpenAiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub async fn call_openai_api(request: OpenAiRequest, api_key: &str) -> Result<OpenAiResponse> {
    let client = Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?
        .json::<OpenAiResponse>()
        .await?;

    Ok(response)
}
