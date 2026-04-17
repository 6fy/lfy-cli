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

/// 默认的 MCP config endpoint（生产 server 地址）
pub const DEFAULT_MCP_CONFIG_ENDPOINT: &str = "https://open-api.6fenyi.com/cli";

/// 本地开发时的 MCP config endpoint（cargo run 时使用）
pub const LOCAL_MCP_CONFIG_ENDPOINT: &str = "http://127.0.0.1:16000";

/// CLI 发布日期
pub const RELEASE_DATE: &str = "26.04.17";

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

/// 返回 MCP config endpoint 地址。
/// - debug 模式：优先环境变量，没设置则用本地地址
/// - release 模式：必须用环境变量，没设置则用生产默认值
pub fn mcp_config_endpoint() -> String {
    if cfg!(debug_assertions) {
        std::env::var(env::MCP_CONFIG_ENDPOINT)
            .unwrap_or_else(|_| LOCAL_MCP_CONFIG_ENDPOINT.to_string())
    } else {
        std::env::var(env::MCP_CONFIG_ENDPOINT)
            .unwrap_or_else(|_| DEFAULT_MCP_CONFIG_ENDPOINT.to_string())
    }
}
