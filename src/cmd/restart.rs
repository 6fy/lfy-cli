use crate::mcp;
use anyhow::Result;
use clap::ArgMatches;
use clap::Args;
use clap::FromArgMatches;

#[derive(Args)]
pub struct RestartArgs {
    #[arg(long, help = "强制从服务器重新拉取 MCP 配置，忽略本地缓存")]
    force: bool,
}

pub async fn handle_restart_cmd(_matches: &ArgMatches) -> Result<()> {
    let _args = RestartArgs::from_arg_matches(_matches)?;

    let spinner = cliclack::spinner();
    spinner.start("正在刷新 MCP 配置...");

    // 强制刷新：先清除本地缓存，再从服务器拉取
    if let Err(e) = mcp::config::fetch_mcp_config().await {
        spinner.stop("MCP 配置刷新失败");
        anyhow::bail!("刷新 MCP 配置失败: {}", e);
    }

    spinner.stop("MCP 配置刷新成功");
    println!("刷新完成 ✅");
    Ok(())
}
