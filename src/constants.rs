use std::path::PathBuf;

pub mod env {
    /// 配置目录，默认 ~/.config/lfy
    pub const CONFIG_DIR: &str = "LFY_CLI_CONFIG_DIR";

    /// MCP config endpoint（init 用于拉取品类 -> URL）
    pub const MCP_CONFIG_ENDPOINT: &str = "LFY_SERVER_URL";

    /// 日志级别
    pub const LOG_LEVEL: &str = "LFY_CLI_LOG_LEVEL";

    /// 日志文件目录
    pub const LOG_FILE: &str = "LFY_CLI_LOG_FILE";
}

/// 默认的 MCP config endpoint（本地 server 地址）
pub const DEFAULT_MCP_CONFIG_ENDPOINT: &str = "https://open-api.6fenyi.com/cli";

/// 返回配置目录（环境变量覆盖或 `~/.config/lfy`）。
pub fn config_dir() -> PathBuf {
    if let Ok(dir) = std::env::var(env::CONFIG_DIR) {
        return PathBuf::from(dir);
    }
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("lfy")
}
