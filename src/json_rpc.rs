use anyhow::Result;
use serde::Serialize;
use serde_json::Value;

use crate::{mcp, settings};

#[derive(Debug, Clone, Serialize)]
struct JsonRpcRequest {
    jsonrpc: &'static str,
    id: String,
    method: String,
    params: Option<Value>,
}

/// 向指定品类的 MCP 端点发送 JSON-RPC 2.0 请求。
pub async fn send(
    category: &str,
    method: &str,
    params: Option<Value>,
    timeout_ms: Option<i32>,
) -> Result<Value> {
    let base = settings::mcp_config_endpoint();
    let mcp_url = format!("{}/{}", base.trim_end_matches('/'), category);

    let body = JsonRpcRequest {
        jsonrpc: "2.0",
        id: mcp::gen_req_id("mcp_rpc"),
        method: method.to_string(),
        params,
    };

    let timeout = std::time::Duration::from_millis(timeout_ms.unwrap_or(30000) as u64);

    let request = reqwest::Client::builder()
        .build()?
        .post(&mcp_url)
        .timeout(timeout)
        .header("Accept", "application/json")
        .json(&body);

    let response = request.send().await.map_err(|err| {
        if err.is_timeout() {
            anyhow::anyhow!("网络请求超时 ({}ms)", timeout.as_millis())
        } else {
            anyhow::anyhow!("网络请求失败 ({})", mcp_url)
        }
    })?;

    let status = response.status();

    if !status.is_success() {
        anyhow::bail!("MCP 请求失败 (HTTP {status})");
    }

    let body_text = response.text().await?;
    let rpc_res = serde_json::from_str::<Value>(&body_text)?;

    Ok(rpc_res)
}
