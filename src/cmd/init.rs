use crate::auth;
use crate::device_id;
use crate::mcp;
use crate::settings;
use anyhow::Result;
use clap::ArgMatches;
use clap::Args;
use clap::FromArgMatches;

#[derive(Args)]
pub struct InitArgs {
    #[arg(long, help = "仅刷新 MCP 配置缓存（不更新 User Key / User Secret）")]
    refresh: bool,

    #[arg(long, help = "User Key（非交互时可传入）")]
    user_key: Option<String>,

    #[arg(long, help = "User Secret（非交互时可传入，注意 shell 历史）")]
    user_secret: Option<String>,

    #[arg(long, help = "MCP Server 地址")]
    server_url: Option<String>,
}

pub async fn handle_init_cmd(matches: &ArgMatches) -> Result<()> {
    let args = InitArgs::from_arg_matches(matches)?;

    // 处理 server_url 配置
    if let Some(url) = &args.server_url {
        let mut settings_data = settings::load_settings();
        settings_data.server_url = Some(url.clone());
        settings::save_settings(&settings_data)?;
        println!("Server 地址已保存: {}", url);
    }

    let non_interactive = args.user_key.is_some() && args.user_secret.is_some();

    if args.refresh {
        let spinner = cliclack::spinner();
        spinner.start("正在刷新 MCP 配置缓存...");
        mcp::config::fetch_mcp_config().await?;
        spinner.stop("MCP 配置刷新成功");
        println!("刷新完成 ✅");
        return Ok(());
    }

    if !non_interactive {
        cliclack::intro("LFY CLI 初始化")?;
    }

    let user_key: String = match args.user_key {
        Some(k) => k,
        None => cliclack::input("User Key")
            .placeholder("请输入 User Key")
            .interact()?,
    };

    let user_secret: String = match args.user_secret {
        Some(s) => s,
        None => cliclack::password("User Secret")
            .mask('*')
            .interact()?,
    };

    // Validate credentials first.
    // When invalid, server returns 401 prompt; we forward the error message to caller.
    let device_id = device_id::get_device_id()?;
    mcp::config::validate_user_credentials(&user_key, &user_secret, &device_id).await?;

    let creds = auth::UserCredentials::new(user_key, user_secret);
    auth::set_credentials(&creds)?;

    let spinner = cliclack::spinner();
    spinner.start("正在拉取 MCP 配置...");
    mcp::config::fetch_mcp_config().await?;
    spinner.stop("MCP 配置拉取成功");

    if non_interactive {
        println!("初始化完成 ✅（已拉取 MCP 配置）");
    } else {
        cliclack::outro("初始化完成 ✅（已拉取 MCP 配置）")?;
    }
    Ok(())
}
