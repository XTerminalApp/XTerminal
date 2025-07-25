use anyhow::Ok;
use reqwest::blocking;
use serde::Serialize;

use crate::llm::deepseek::DeepSeekRequest;

pub mod claude;
pub mod deepseek;
pub mod openai;

pub const CURRENT_ENV: &str = "CurrentEnvVars: ";
pub const SHELL_HISTORY: &str = "ShellHistory: ";
pub const NOTES: &str =
    "Notes: Please put the final command in last single line and wrap it with <<>>";

#[allow(async_fn_in_trait)]
pub trait IsLLMRequest {
    fn send_request(
        &self,
        api_key: &str,
        messages: &[Message],
    ) -> anyhow::Result<blocking::Response>;
}

#[derive(Debug, Copy, Clone, Default, Serialize)]
pub enum LLMModel {
    #[default]
    #[serde(rename = "deepseek-chat")]
    DeepSeekChat,
    #[serde(rename = "deepseek-reasoner")]
    DeepSeekReasoner,
}
impl From<&str> for LLMModel {
    fn from(model_name: &str) -> Self {
        match model_name {
            "deepseek-chat" => LLMModel::DeepSeekChat,
            "deepseek-reasoner" => LLMModel::DeepSeekReasoner,
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

// pub trait LLM {
//     fn get_api_key(&self) -> &str;
// }

pub fn build_and_send_request(
    model_type: LLMModel,
    api_key: &str,
    messages: &[Message],
) -> anyhow::Result<blocking::Response> {
    let response = match model_type {
        LLMModel::DeepSeekChat | LLMModel::DeepSeekReasoner => {
            let request = DeepSeekRequest::build(model_type, messages.to_vec(), false);
            request.send_request(api_key, messages)?
        } // _ => return Err(anyhow::anyhow!("Unsupported model type")),
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
        } // _ =>
    };
    Ok(llm_reply.to_string())
}
