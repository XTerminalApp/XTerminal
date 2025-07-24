use std::process::Command;

use clap::Parser;
use llm::{IsLLMRequest, LLMModel, deepseek::DeepSeekRequest};
use rustyline::{Editor, error::ReadlineError};

use crate::llm::Message;

const MESSAGE_BEGIN: &str = "You are a great helper for learning algorithms.";
pub const READLINE_PROMPT_BASE: &str = "Axec> ";
pub const RUNNING: &str = "|/-\\";

pub mod cli;
pub mod llm;

pub fn run() {
    let cli = cli::Cli::parse().load_config();
    let mut messages = Vec::new();
    messages.push(Message::from_system(MESSAGE_BEGIN));
    let mut editor = Editor::<(), _>::new().expect("Failed to create editor");
    println!("Welcome to Axec!");

    let deepseek = cli.get_deepseek().expect("No deepseek configured");
    let deepseek_apikey = deepseek
        .api_key
        .clone()
        .expect("No deepseek apikey configured");

    loop {
        let readline = editor.readline(READLINE_PROMPT_BASE);
        match readline {
            Ok(user_input) => match try_execute(&user_input) {
                Ok(output) => println!("{}", output),
                Err(_err) => {
                    println!("Looks like input is not a valid command, so sent it to LLM\n");
                    let deepseek_req =
                        DeepSeekRequest::build(LLMModel::DeepSeekChat, messages.clone(), false);
                    messages.push(Message::from_user(&user_input));
                    let response = deepseek_req.send_request(&deepseek_apikey, &messages);
                    let json: serde_json::Value = response.unwrap().json().unwrap();
                    let llm_reply = json["choices"][0]["message"]["content"].as_str().unwrap();
                    println!("{}", llm_reply)
                }
            },
            Err(ReadlineError::Eof) => {
                println!("Exiting Axec");
                break;
            }
            Err(_) => {
                println!("Error reading input");
            }
        }
    }
}

fn try_execute(line: &str) -> Result<String, String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return Err("Empty command".to_string());
    }

    let (cmd, args) = parts.split_first().unwrap();

    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute command: {e}"))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
