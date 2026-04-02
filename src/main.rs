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
            cmd::init::InitArgs::augment_args(Command::new("init")).about("初始化并保存 User Key / User Secret"),
        );

    // Standalone command: show current machine device_id.
    cmd = cmd.subcommand(Command::new("stats").about("查看当前机器的 device_id"));

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
        Some(("init", matches)) => cmd::init::handle_init_cmd(matches).await,
        Some(("stats", matches)) => cmd::stats::handle_stats_cmd(matches).await,
        Some((category, matches)) => cmd::call::handle_call_cmd(category, matches).await,
        _ => anyhow::bail!("未知命令"),
    }
}
