use serde::{Deserialize, Serialize};

use crate::constants::{config_dir, env, DEFAULT_MCP_CONFIG_ENDPOINT};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server_url: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { server_url: None }
    }
}

pub fn settings_path() -> std::path::PathBuf {
    config_dir().join("settings.json")
}

pub fn load_settings() -> Settings {
    let path = settings_path();
    if let Ok(data) = std::fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Settings::default()
    }
}

pub fn save_settings(settings: &Settings) -> anyhow::Result<()> {
    use std::io::Write;
    let path = settings_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::File::create(&path)?;
    write!(file, "{}", serde_json::to_string_pretty(settings)?)?;
    Ok(())
}

/// 返回 MCP config endpoint，优先级：环境变量 LFY_SERVER_URL > 配置文件 > 默认值
pub fn mcp_config_endpoint() -> String {
    // 1. 环境变量（最高优先）
    if let Ok(url) = std::env::var(env::MCP_CONFIG_ENDPOINT) {
        if !url.is_empty() {
            return url;
        }
    }

    // 2. 配置文件
    let settings = load_settings();
    if let Some(url) = settings.server_url {
        if !url.is_empty() {
            return url;
        }
    }

    // 3. 默认值
    DEFAULT_MCP_CONFIG_ENDPOINT.to_string()
}
