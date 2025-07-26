use std::fs;
use std::process::{self, Command};

use clap::Parser;
use colored::*;
use crossterm::style::Attribute::*;
use crossterm::style::Color::*;
use llm::LLMModel;
use rustyline::error::ReadlineError;
use termimad::rgb;
use termimad::{CompoundStyle, MadSkin};

use crate::llm::{Message, build_and_send_request, parse_response};

pub const DEFAULT_CONFIG: &str = r#"[general]
model_name = "deepseek-chat"
api_key = "sk-2f2d2bf56d0247a2922f68cc67eea799"
stream = false
"#;

pub const SYSTEM_PROMPT: &str = r#"You are a great helper for learning algorithms & CLI commands.
Please fully use markdown format when outputing.
Please put file name (the algorithm or command name user inputed) in the first single line, which must be fully lowercased with `_` inside.
Statements and explanation must be commented.

See the two example:
```shell
# some_command
sudo some_command
```

```somelanguage
// algorithm_name.somelanguagesuffix
int algorithm_name(const vector<int>& arr, int target) {
    ...
}
```"#;

pub const READLINE_PROMPT_BASE: &str = "Axec> ";
pub const RUNNING: &str = "|/-\\";

pub mod cli;
pub mod llm;

pub fn run() -> anyhow::Result<String> {
    // 创建一个美观的 MadSkin 配置
    let mut skin = MadSkin::no_style();

    let code_fg = rgb(110, 110, 205);
    let text_fg = rgb(190, 230, 80);

    // 自定义样式：现代化的配色方案
    skin.set_fg(text_fg);

    skin.code_block.set_fg(code_fg);
    skin.inline_code.set_fg(code_fg);

    skin.italic.set_fg(Magenta);
    skin.strikeout = CompoundStyle::new(Some(Red), None, Bold.into()); // 删除线为红色加粗

    let cli = cli::Cli::parse().load_config();
    let mut messages = Vec::new();
    messages.push(Message::from_system(SYSTEM_PROMPT));
    let mut editor = rustyline::DefaultEditor::new()?;
    let start_text = "A CLI tool for learning algorithms & terminal commands.";
    println!("Axec: {}", start_text.green().bold());

    let general = cli.get().expect("No LLM configured");
    let model_type: LLMModel = general.model_name.as_str().into();
    let stream = general.stream;

    loop {
        let readline = editor.readline(READLINE_PROMPT_BASE);
        match readline {
            Ok(user_input) => match try_execute(user_input.trim()) {
                Ok(output) => println!("{output}"),
                Err(_err) => {
                    let sent_to_llm = "Looks like input is not a valid command, now sent it to LLM";
                    println!("\n{}\n", sent_to_llm.purple().bold());
                    if messages.len() > 1 {
                        messages.pop();
                    }
                    messages.push(Message::from_user(&user_input));
                    let response =
                        build_and_send_request(model_type, &general.api_key, &messages, stream);
                    let llm_reply = parse_response(model_type, response)?;
                    skin.print_text(&llm_reply);
                    let algorithm_name = process_algorithm_competition(&llm_reply);
                    fs::write(format!("{algorithm_name}.md"), &llm_reply).unwrap();
                }
            },
            Err(ReadlineError::Eof) => {
                let exit_text = "Exiting Axec\nBye Bye!";
                println!("{}", exit_text.yellow().bold());
                process::exit(0)
            }
            Err(ReadlineError::Interrupted) => {
                let interrupted_text = "Use Control D to exit";
                println!("{}", interrupted_text.red().bold());
            }
            Err(_) => {
                println!("Error reading input");
            }
        }
    }
}

fn try_execute(line: &str) -> anyhow::Result<String> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.is_empty() {
        return anyhow::Ok(String::new());
    }
    let (cmd, args) = parts.split_first().unwrap();
    let output = Command::new(cmd).args(args).output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

// quick_srot
// # quick_sort
fn process_algorithm_competition(input: &str) -> String {
    let input = input.trim().to_string();
    let mut iter = input.lines();
    iter.next().unwrap();
    let second = iter.next().unwrap();

    let ret = second.replace(['/', ' ', '#'], "");

    if ret.is_empty() {
        String::from("unknown_algorithm")
    } else {
        ret
    }
}
