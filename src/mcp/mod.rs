use anyhow::Result;
use rand::Rng;

pub(crate) mod config;

/// 获取给定 `category` 对应的 MCP HTTP 端点 URL。
///
/// 行为：
/// - 优先读取本地缓存的 MCP 配置（`mcp_config.enc`）
/// - 缓存缺失时自动调用服务端 `mcp/config` 拉取并缓存
pub async fn get_mcp_url(category: &str) -> Result<String> {
    config::get_mcp_url(category).await
}

/// 生成请求 ID：`{prefix}_{timestamp_ms}_{random_hex}`。
pub fn gen_req_id(prefix: &str) -> String {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let random = generate_random_hex(8);
    format!("{prefix}_{timestamp}_{random}")
}

fn generate_random_hex(length: usize) -> String {
    let byte_len = (length + 1) / 2;
    let bytes: Vec<u8> = (0..byte_len).map(|_| rand::rng().random::<u8>()).collect();
    let hex = hex::encode(bytes);
    hex[..length.min(hex.len())].to_string()
}
