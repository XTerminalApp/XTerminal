use reqwest::blocking;
use serde::Serialize;

use crate::{
    SYSTEM_PROMPT,
    llm::{IsLLMRequest, LLMModel, Message},
};

pub const CLAUDE_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";

#[derive(Serialize, Debug, Clone)]
pub struct ClaudeRequest {
    model: LLMModel,
    messages: Vec<Message>,
    max_tokens: u32, // Claude要求max_tokens为必填字段
    #[serde(default = "default_system_prompt")]
    system: String, // Claude的system提示单独作为一个字段
    stream: bool,
}

#[allow(dead_code)]
fn default_system_prompt() -> String {
    SYSTEM_PROMPT.to_string()
}

impl ClaudeRequest {
    pub const fn build(
        model: LLMModel,
        messages: Vec<Message>,
        system: String,
        stream: bool,
    ) -> Self {
        if matches!(
            model,
            LLMModel::DeepSeekChat
                | LLMModel::DeepSeekReasoner
                | LLMModel::Gpt4
                | LLMModel::Gpt4O
                | LLMModel::Gpt4Turbo
        ) {
            panic!("Wrong model name for Claude API");
        }
        Self {
            model,
            messages,
            max_tokens: u32::MAX,
            system,
            stream,
        }
    }
}

impl IsLLMRequest for ClaudeRequest {
    fn send_request(&self, api_key: &str) -> anyhow::Result<blocking::Response> {
        let client = reqwest::blocking::Client::new();

        let response = client
            .post(CLAUDE_ENDPOINT)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01") // Claude API要求版本头
            .header("Content-Type", "application/json")
            .json(&self)
            .send()?;

        Ok(response)
    }
}
