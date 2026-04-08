use crate::mcp;
use anyhow::Result;
use clap::ArgMatches;

pub async fn handle_upgrade_cmd(_matches: &ArgMatches) -> Result<()> {
    let spinner = cliclack::spinner();
    spinner.start("正在刷新 MCP 配置...");

    if let Err(e) = mcp::config::fetch_mcp_config().await {
        spinner.stop("MCP 配置刷新失败");
        anyhow::bail!("刷新 MCP 配置失败: {}", e);
    }

    spinner.stop("MCP 配置刷新成功");
    println!("刷新完成 ✅");
    Ok(())
}