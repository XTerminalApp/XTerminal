use std::{env, fs, path::PathBuf};

use clap::Parser;
use serde::Deserialize;
use toml;

use crate::DEFAULT_CONFIG;

// TOML配置文件对应的结构体
#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: General,
}

#[derive(Debug, Deserialize)]
pub struct General {
    pub model_name: String,
    pub api_key: String,
    #[serde(default = "default_stream")]
    pub stream: bool,
}
pub const fn default_stream() -> bool {
    false
}

// Cli结构体，包含Clap字段和配置文件
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// 可选的配置文件路径
    // #[arg(short, long, default_value = "axec.toml")]
    // config: String,

    // 存储解析后的配置文件
    #[clap(skip)]
    pub config_data: Option<Config>,
}

impl Cli {
    // 加载和解析TOML配置文件
    pub fn load_config(mut self) -> Self {
        let config_dir = get_config_dir().unwrap();
        let config_path = config_dir.join("config.toml");

        if !config_dir.exists() {
            fs::create_dir_all(config_dir).unwrap()
        }

        if !config_path.exists() {
            fs::write(&config_path, DEFAULT_CONFIG).unwrap()
        }

        let config_content =
            fs::read_to_string(&config_path).unwrap_or_else(|e| panic!("fail to read file: {e}"));

        self.config_data =
            toml::from_str(&config_content).unwrap_or_else(|e| panic!("config error: {e}"));
        self
    }

    // 获取selected值
    pub fn get(&self) -> Option<&General> {
        self.config_data.as_ref().map(|config| &config.general)
    }
}
fn get_config_dir() -> anyhow::Result<PathBuf> {
    let path = if cfg!(target_os = "windows") {
        PathBuf::from(env::var("USERPROFILE")?)
            .join("AppData")
            .join("Roaming")
    } else {
        PathBuf::from(env::var("HOME")?).join(".config")
    };

    Ok(path.join("axec"))
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

        assert_eq!(
            "sk-2f2d2bf56d0247a2922f68cc67eea799",
            config_data.general.api_key
        );
        assert_eq!("deepseek-chat", config_data.general.model_name);
    }
}
