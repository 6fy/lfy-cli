use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{crypto, fs_util};

use super::gen_req_id;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfigItem {
    pub url: String,
    #[serde(rename = "type")]
    pub transport_type: Option<String>,
    #[serde(default)]
    pub is_authed: Option<bool>,
    pub biz_type: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetMcpConfigResponse {
    pub errcode: i32,
    pub errmsg: String,
    #[serde(default)]
    pub list: Vec<McpConfigItem>,
}

fn mcp_config_path() -> std::path::PathBuf {
    crate::constants::config_dir().join("mcp_config.enc")
}

pub fn load_cached_mcp_config() -> Option<Vec<McpConfigItem>> {
    let data = std::fs::read(mcp_config_path()).ok()?;
    crypto::try_decrypt_data::<Vec<McpConfigItem>>(&data).ok()
}

pub fn save_mcp_config(items: &[McpConfigItem]) -> Result<()> {
    let key = crypto::load_existing_key().unwrap_or_else(|| {
        let k = crypto::generate_random_key();
        tracing::info!("已生成新的加密密钥（用于 MCP 配置缓存）");
        k
    });

    crypto::save_key(&key)?;

    let encrypted = crypto::encrypt_data(items, &key)?;
    let path = mcp_config_path();
    fs_util::atomic_write(&path, &encrypted, Some(0o600))?;
    tracing::info!("MCP 配置缓存已保存到 {}", path.display());
    Ok(())
}

async fn fetch_mcp_config_from_server() -> Result<GetMcpConfigResponse> {
    // JSON-RPC endpoint: we reuse the base URL; server accepts all paths.
    let endpoint = crate::settings::mcp_config_endpoint();

    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": gen_req_id("mcp_config"),
        "method": "mcp/config",
        "params": null
    });

    let resp = reqwest::Client::builder()
        .build()?
        .post(&endpoint)
        .json(&body)
        .header("Accept", "application/json")
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|err| anyhow::anyhow!("MCP config request failed: {err}"))?;

    let status = resp.status();
    if !status.is_success() {
        anyhow::bail!("MCP config request failed (HTTP {status})");
    }

    let rpc_res: Value = resp.json().await?;
    let result = rpc_res
        .get("result")
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("MCP config response missing `result`: {rpc_res}"))?;

    let parsed: GetMcpConfigResponse = serde_json::from_value(result)?;
    Ok(parsed)
}

pub async fn fetch_mcp_config() -> Result<Vec<McpConfigItem>> {
    let resp = fetch_mcp_config_from_server().await?;
    if resp.errcode != 0 {
        anyhow::bail!("获取 MCP 配置失败：[{}] {}", resp.errcode, resp.errmsg);
    }
    save_mcp_config(&resp.list)?;
    Ok(resp.list)
}

async fn validate_user_credentials_on_server(
    user_key: &str,
    user_secret: &str,
    device_id: &str,
) -> Result<()> {
    let endpoint = crate::settings::mcp_config_endpoint();

    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": gen_req_id("mcp_auth_validate"),
        "method": "mcp/auth/validate",
        "params": {
            "user_key": user_key,
            "user_secret": user_secret,
            "device_id": device_id,
        }
    });

    let resp = reqwest::Client::builder()
        .build()?
        .post(&endpoint)
        .json(&body)
        .header("Accept", "application/json")
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|err| anyhow::anyhow!("MCP鉴权请求失败: {err}"))?;

    let status = resp.status();
    if !status.is_success() {
        anyhow::bail!("MCP鉴权请求失败 (HTTP {status})");
    }

    let rpc_res: Value = resp.json().await?;

    if let Some(result) = rpc_res.get("result") {
        if let Some(ok) = result.get("ok").and_then(|v| v.as_bool()) {
            if ok {
                return Ok(());
            }

            // Check error message to determine error type
            if let Some(msg) = result.get("error").and_then(|v| v.as_str()) {
                if msg == "当前设备未授权" {
                    anyhow::bail!("当前设备未授权");
                }
                anyhow::bail!("您的个人Key或Secret已失效或不正确。");
            }

            anyhow::bail!("鉴权失败");
        }
    }

    // Fallback: JSON-RPC error object
    if let Some(msg) = rpc_res
        .get("error")
        .and_then(|e| e.get("message"))
        .and_then(|v| v.as_str())
    {
        if msg == "当前设备未授权" {
            anyhow::bail!("当前设备未授权");
        }
        anyhow::bail!("您的个人Key或Secret已失效或不正确。");
    }

    anyhow::bail!("鉴权失败：{rpc_res}");
}

/// Validate user_key/user_secret/device_id by calling server `mcp/auth/validate`.
///
/// On invalid credentials, this returns an error whose message is exactly the server's
/// 401 prompt (e.g. `您的个人Key或Secret已失效或不正确。`).
pub async fn validate_user_credentials(user_key: &str, user_secret: &str, device_id: &str) -> Result<()> {
    validate_user_credentials_on_server(user_key, user_secret, device_id).await
}

pub async fn get_mcp_url(category: &str) -> Result<String> {
    let category = category.to_string();

    // 1) Prefer cached config.
    if let Some(list) = load_cached_mcp_config() {
        if let Some(item) = list.into_iter().find(|i| i.biz_type == category) {
            return Ok(item.url);
        }
    }

    // 2) Cache miss: fetch from server once.
    let list = fetch_mcp_config().await?;
    let item = list
        .into_iter()
        .find(|i| i.biz_type == category)
        .ok_or_else(|| anyhow::anyhow!("当前企业暂不支持 {} 命令", category))?;

    Ok(item.url)
}

