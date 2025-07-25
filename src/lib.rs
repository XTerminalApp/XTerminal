use anyhow::anyhow;
use clap::Parser;
use llm::LLMModel;
use rustyline::error::ReadlineError;
use std::process::Command;

use crate::llm::{Message, build_and_send_request, parse_response};

const MESSAGE_BEGIN: &str = "You are a great helper for learning algorithms.";
pub const READLINE_PROMPT_BASE: &str = "Axec> ";
pub const RUNNING: &str = "|/-\\";

pub mod cli;
pub mod llm;

pub fn run() -> anyhow::Result<String> {
    let cli = cli::Cli::parse().load_config();
    let mut messages = Vec::new();
    messages.push(Message::from_system(MESSAGE_BEGIN));
    let mut editor = rustyline::DefaultEditor::new()?;
    println!("Welcome to Axec!");

    let general = cli.get().expect("No LLM configured");
    let model_type: LLMModel = general.model_name.as_str().into();

    loop {
        let readline = editor.readline(READLINE_PROMPT_BASE);
        match readline {
            Ok(user_input) => match try_execute(&user_input) {
                Ok(output) => println!("{output}"),
                Err(_err) => {
                    println!("Looks like input is not a valid command, now sent it to LLM\n");
                    messages.push(Message::from_user(&user_input));
                    let response = build_and_send_request(model_type, &general.api_key, &messages);
                    let llm_reply = parse_response(model_type, response)?;
                    println!("{llm_reply}")
                }
            },
            Err(ReadlineError::Eof) => {
                println!("Exiting Axec");
                return Err(anyhow!("EOF"));
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
