use crate::auth;
use crate::device_id;
use crate::mcp;
use anyhow::Result;
use clap::ArgMatches;
use clap::Args;
use clap::FromArgMatches;

#[derive(Args)]
pub struct LoginArgs {
    #[arg(long, help = "User Key（非交互时可传入）")]
    user_key: Option<String>,

    #[arg(long, help = "User Secret（非交互时可传入，注意 shell 历史）")]
    user_secret: Option<String>,
}

pub async fn handle_login_cmd(matches: &ArgMatches) -> Result<()> {
    let args = LoginArgs::from_arg_matches(matches)?;

    let non_interactive = args.user_key.is_some() && args.user_secret.is_some();

    if !non_interactive {
        cliclack::intro("LFY CLI 登录")?;
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

    let device_id = device_id::get_device_id()?;
    mcp::config::validate_user_credentials(&user_key, &user_secret, &device_id).await?;

    let creds = auth::UserCredentials::new(user_key, user_secret);
    auth::set_credentials(&creds)?;

    let spinner = cliclack::spinner();
    spinner.start("正在拉取 MCP 配置...");
    mcp::config::fetch_mcp_config().await?;
    spinner.stop("MCP 配置拉取成功");

    if non_interactive {
        println!("登录完成 ✅（已拉取 MCP 配置）");
    } else {
        cliclack::outro("登录完成 ✅（已拉取 MCP 配置）")?;
    }
    Ok(())
}