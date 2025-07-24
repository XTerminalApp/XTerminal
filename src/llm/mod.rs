use reqwest::blocking;
use serde::Serialize;

pub mod claude;
pub mod deepseek;
pub mod openai;

pub const CURRENT_ENV: &str = "CurrentEnvVars: ";
pub const SHELL_HISTORY: &str = "ShellHistory: ";
pub const NOTES: &str =
    "Notes: Please put the final command in last single line and wrap it with <<>>";

// pub static RESTORED_MESSAGES: LazyLock<RwLock<Vec<Message>>> =
//     LazyLock::new(|| RwLock::new(Vec::new()));

#[allow(async_fn_in_trait)]
pub trait IsLLMRequest {
    fn send_request(
        &self,
        api_key: &str,
        messages: &[Message],
    ) -> anyhow::Result<blocking::Response>;
}

#[derive(Debug, Clone, Default, Serialize)]
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
