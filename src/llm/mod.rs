use anyhow::Ok;
use reqwest::blocking;
use serde::Serialize;

use crate::{
    SYSTEM_PROMPT,
    llm::{claude::ClaudeRequest, deepseek::DeepSeekRequest, openai::OpenaiRequest},
};

pub mod claude;
pub mod deepseek;
pub mod openai;

pub const CURRENT_ENV: &str = "CurrentEnvVars: ";
pub const SHELL_HISTORY: &str = "ShellHistory: ";
#[allow(async_fn_in_trait)]
pub trait IsLLMRequest {
    fn send_request(&self, api_key: &str) -> anyhow::Result<blocking::Response>;
}

#[derive(Debug, Copy, Clone, Default, Serialize)]
pub enum LLMModel {
    #[default]
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
    #[serde(rename = "gpt-4")]
    Gpt4,
    #[serde(rename = "gpt-4o")]
    Gpt4O,
    #[serde(rename = "gpt-4-turbo")]
    Gpt4Turbo,
    #[serde(rename = "claude-3-opus-20240229")]
    Claude3Opus,
    #[serde(rename = "claude-3-sonnet-20240229")]
    Claude3Sonnet,
    #[serde(rename = "claude-3-haiku-20240307")]
    Claude3Haiku,
}
impl From<&str> for LLMModel {
    fn from(model_name: &str) -> Self {
        match model_name {
            "deepseek-chat" => LLMModel::DeepSeekChat,
            "deepseek-reasoner" => LLMModel::DeepSeekReasoner,
            "gpt-4" => LLMModel::Gpt4,
            "gpt-4o" => LLMModel::Gpt4O,
            "gpt-4-turbo" => LLMModel::Gpt4Turbo,
            _ => panic!("No matched model!"),
        }
    }
}

#[derive(Clone, Debug, Serialize, Default)]
pub struct Message {
    role: Role,
    content: String,
}

impl Message {
    pub const fn new(role: Role, content: String) -> Self {
        Message { role, content }
    }

    /// User's message.
    pub fn from_user(once_serialized_input: &str) -> Self {
        Message {
            content: once_serialized_input.to_string(),
            role: Role::User,
        }
    }
    pub fn from_assistant(once_serialized_input: &str) -> Self {
        Message {
            content: once_serialized_input.to_string(),
            role: Role::Assistant,
        }
    }
    pub fn from_system(once_serialized_input: &str) -> Self {
        Message {
            content: once_serialized_input.to_string(),
            role: Role::System,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Assistant,
    System,
    #[default]
    User,
}

pub fn build_and_send_request(
    model_type: LLMModel,
    api_key: &str,
    messages: &[Message],
    stream: bool,
) -> anyhow::Result<blocking::Response> {
    let response = match model_type {
        LLMModel::DeepSeekChat | LLMModel::DeepSeekReasoner => {
            let request = DeepSeekRequest::build(model_type, messages.to_vec(), stream);
            request.send_request(api_key)?
        }

        LLMModel::Gpt4 | LLMModel::Gpt4Turbo | LLMModel::Gpt4O => {
            let request = OpenaiRequest::build(model_type, messages.to_vec(), stream);
            request.send_request(api_key)?
        }

        LLMModel::Claude3Opus | LLMModel::Claude3Haiku | LLMModel::Claude3Sonnet => {
            let request = ClaudeRequest::build(
                model_type,
                messages.to_vec(),
                SYSTEM_PROMPT.to_string(),
                stream,
            );
            request.send_request(api_key)?
        }
    };
    Ok(response)
}

pub fn parse_response(
    model_type: LLMModel,
    response: anyhow::Result<blocking::Response>,
) -> anyhow::Result<String> {
    let json: serde_json::Value = response?.json()?;
    let llm_reply = match model_type {
        LLMModel::DeepSeekChat | LLMModel::DeepSeekReasoner => {
            json["choices"][0]["message"]["content"].as_str().unwrap()
        }
        LLMModel::Gpt4O | LLMModel::Gpt4Turbo | LLMModel::Gpt4 => {
            json["choices"][0]["message"]["content"].as_str().unwrap()
        }
        LLMModel::Claude3Opus | LLMModel::Claude3Haiku | LLMModel::Claude3Sonnet => {
            json["choices"][0]["message"]["content"].as_str().unwrap()
        }
    };
    Ok(llm_reply.to_string())
}
