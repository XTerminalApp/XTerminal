use std::time::Duration;

use crate::llm::{IsLLMRequest, LLMModel, Message};

use reqwest::blocking;
use serde::Serialize;

pub const DEEPSEEK_ENDPOINT: &str = "https://api.deepseek.com/chat/completions";

#[derive(Serialize, Debug, Clone)]
pub struct DeepSeekRequest {
    model: LLMModel,
    messages: Vec<Message>,
    stream: bool,
}
impl DeepSeekRequest {
    pub const fn build(model: LLMModel, messages: Vec<Message>, stream: bool) -> Self {
        if matches!(
            model,
            LLMModel::Gpt4O | LLMModel::Gpt4 | LLMModel::Gpt4Turbo
        ) {
            panic!("Wrong model name");
        }
        Self {
            model,
            messages,
            stream,
        }
    }
}
impl IsLLMRequest for DeepSeekRequest {
    fn send_request(&self, api_key: &str) -> anyhow::Result<blocking::Response> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(128))
            .build()?;

        let response = client
            .post(DEEPSEEK_ENDPOINT)
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&self)
            .send()?;

        Ok(response)
    }
}
