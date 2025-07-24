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
        let model = match model {
            LLMModel::DeepSeekChat => model,
            LLMModel::DeepSeekReasoner => model,
            // _ => panic!("It is not a DeepSeek model!"),
        };
        Self {
            model,
            messages,
            stream,
        }
    }
}
impl IsLLMRequest for DeepSeekRequest {
    fn send_request(
        &self,
        api_key: &str,
        messages: &[Message],
    ) -> anyhow::Result<blocking::Response> {
        let deepseek_request =
            DeepSeekRequest::build(Default::default(), messages.to_vec(), Default::default());
        let client = reqwest::blocking::Client::new();

        let response = client
            .post(DEEPSEEK_ENDPOINT)
            .header("Authorization", format!("Bearer {api_key}"))
            .header("Content-Type", "application/json")
            .json(&deepseek_request)
            .send()?;

        Ok(response)
    }
}
