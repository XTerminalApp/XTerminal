use std::process::Command;
use std::process::exit;

use clap::Parser;
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
Please fully use markdown format, algorithm name or command name must be the first line with '####', comments and unrelated statements must be commented.
See the two example:

```shell
# install_something_on_debian
sudo apt install something
```

```cpp
// algorithm_name
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
    println!("Axec: A CLI tool for learning algorithms & terminal commands.");

    let general = cli.get().expect("No LLM configured");
    let model_type: LLMModel = general.model_name.as_str().into();
    let stream = general.stream;

    loop {
        let readline = editor.readline(READLINE_PROMPT_BASE);
        match readline {
            Ok(user_input) => match try_execute(user_input.trim()) {
                Ok(output) => println!("{output}"),
                Err(_err) => {
                    println!("\nLooks like input is not a valid command, now sent it to LLM\n");
                    if messages.len() > 1 {
                        messages.pop();
                    }
                    messages.push(Message::from_user(&user_input));
                    let response =
                        build_and_send_request(model_type, &general.api_key, &messages, stream);
                    let llm_reply = parse_response(model_type, response)?;
                    skin.print_text(&llm_reply);
                }
            },
            Err(ReadlineError::Eof) => {
                println!("Exiting Axec\nBye Bye!");
                exit(0)
            }
            Err(ReadlineError::Interrupted) => {
                println!("Use Control D to exit")
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
