mod auth;
mod cmd;
mod config;
mod constants;
mod crypto;
mod fs_util;
mod help;
mod json_rpc;
mod logging;
mod mcp;
mod device_id;
mod settings;

use anyhow::Result;
use clap::Args;
use clap::Command;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("\x1b[31mError: {}\x1b[0m", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    dotenvy::dotenv().ok();

    logging::init_logging();

    let categories = config::get_categories();

    let mut cmd = Command::new(env!("CARGO_BIN_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("陆份仪 LFY CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .disable_help_subcommand(true)
        .subcommand(
            cmd::login::LoginArgs::augment_args(Command::new("login")).about("登录陆份仪官方系统，完成账号绑定"),
        );

    // Standalone command: show current machine device_id.
    cmd = cmd.subcommand(Command::new("status").about("查看状态"));

    // Standalone command: refresh MCP config cache.
    cmd = cmd.subcommand(Command::new("upgrade").about("升级工具，为智能体扩展能力"));

    for category in categories.iter() {
        cmd = cmd.subcommand(cmd::call::CallArgs::augment_args(
            Command::new(category.name)
                .about(category.description)
                .disable_help_subcommand(true)
                .disable_help_flag(true),
        ));
    }

    let matches = cmd.get_matches();

    match matches.subcommand() {
        Some(("login", matches)) => cmd::login::handle_login_cmd(matches).await,
        Some(("status", matches)) => cmd::status::handle_status_cmd(matches).await,
        Some(("upgrade", matches)) => cmd::upgrade::handle_upgrade_cmd(matches).await,
        Some((category, matches)) => cmd::call::handle_call_cmd(category, matches).await,
        _ => anyhow::bail!("未知命令"),
    }
}
