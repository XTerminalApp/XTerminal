use std::time::Duration;

use reqwest::blocking;
use serde::Serialize;

use crate::llm::{IsLLMRequest, LLMModel, Message};

pub const OPENAI_ENDPOINT: &str = "https://api.openai.com/v1";

#[derive(Serialize, Debug, Clone)]
pub struct OpenaiRequest {
    model: LLMModel,
    messages: Vec<Message>,
    stream: bool,
}

impl OpenaiRequest {
    pub const fn build(model: LLMModel, messages: Vec<Message>, stream: bool) -> Self {
        if matches!(model, LLMModel::DeepSeekChat | LLMModel::DeepSeekReasoner) {
            panic!("Wrong model name");
        }
        Self {
            model,
            messages,
            stream,
        }
    }
}

impl IsLLMRequest for OpenaiRequest {
    fn send_request(&self, api_key: &str) -> anyhow::Result<blocking::Response> {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(64))
            .build()?;

        let response = client
            .post(OPENAI_ENDPOINT)
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&self)
            .send()?;

        Ok(response)
    }
}
