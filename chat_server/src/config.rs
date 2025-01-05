use std::{env, fs::File};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}
impl AppConfig {
    pub fn load() -> Result<Self> {
        // 读取 ./app.yaml , /etc/chat-rs/app.yaml, env CHAT_RS_APP_CONFIG
        let ret = match (
            File::open("./app.yaml"),
            File::open("/etc/chat-rs/app.yaml"),
            env::var("CHAT_RS_APP_CONFIG"),
        ) {
            (Ok(file), _, _) => file,
            (_, Ok(file), _) => file,
            (_, _, Ok(path)) => File::open(path)?,
            _ => anyhow::bail!("Failed to load app.yaml"),
        };
        let config: AppConfig = serde_yaml::from_reader(ret)?;
        Ok(config)
    }
}
