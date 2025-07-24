use std::fs;

use clap::Parser;
use serde::Deserialize;
use toml;

// TOML配置文件对应的结构体
#[derive(Debug, Deserialize)]
pub struct Config {
    general: Option<General>,
    deepseek: Option<Deepseek>,
    openai: Option<Openai>,
    claude: Option<Claude>,
}

#[derive(Debug, Deserialize)]
struct General {
    selected: String,
}

#[derive(Debug, Deserialize)]
pub struct Deepseek {
    pub name: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Openai {
    name: Option<String>,
    api_key: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Claude {
    name: Option<String>,
    api_key: Option<String>,
}

// Cli结构体，包含Clap字段和配置文件
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// 可选的配置文件路径
    #[arg(short, long, default_value = "axec.toml")]
    config: String,

    // 存储解析后的配置文件
    #[clap(skip)]
    pub config_data: Option<Config>,
}

impl Cli {
    // 加载和解析TOML配置文件
    pub fn load_config(mut self) -> Self {
        let config_content =
            fs::read_to_string(&self.config).unwrap_or_else(|e| panic!("fail to read file: {e}"));

        self.config_data =
            toml::from_str(&config_content).unwrap_or_else(|e| panic!("config error: {e}"));
        self
    }

    // 获取selected值
    pub fn get_selected(&self) -> Option<&str> {
        self.config_data
            .as_ref()
            .and_then(|config| config.general.as_ref())
            .map(|general| general.selected.as_ref())
    }

    // 获取deepseek配置
    pub fn get_deepseek(&self) -> Option<&Deepseek> {
        self.config_data
            .as_ref()
            .and_then(|config| config.deepseek.as_ref())
    }

    // 获取openai配置
    pub fn get_openai(&self) -> Option<&Openai> {
        self.config_data
            .as_ref()
            .and_then(|config| config.openai.as_ref())
    }

    // 获取claude配置
    pub fn get_claude(&self) -> Option<&Claude> {
        self.config_data
            .as_ref()
            .and_then(|config| config.claude.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_config_file() {
        fs::read_to_string("axec.toml").expect("Cannot find file");
    }
    #[test]
    fn read_and_parse_config_file() {
        let config_content = fs::read_to_string("axec.toml").expect("Cannot find file");

        let config_data: Config =
            toml::from_str(&config_content).unwrap_or_else(|e| panic!("config error: {e}"));

        let selected = config_data
            .general
            .as_ref()
            .map(|general| general.selected.as_str());

        assert_eq!(
            Some("sk-2f2d2bf56d0247a2922f68cc67eea799".to_string()),
            config_data.deepseek.and_then(|deepseek| deepseek.api_key)
        );
        assert_eq!(Some("deepseek"), selected);
    }
}
