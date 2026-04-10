use crate::constants::RELEASE_DATE;
use crate::device_id;
use crate::settings::mcp_config_endpoint;

use anyhow::Result;
use clap::ArgMatches;

pub async fn handle_status_cmd(_matches: &ArgMatches) -> Result<()> {
    let id = device_id::get_device_id()?;
    let version = env!("CARGO_PKG_VERSION");
    // 冒号位置固定，key 左对齐，value 从同一列开始显示
    let max_key_width = "release date".len();
    println!("{:<width$}: v{}", "version", version, width = max_key_width);
    println!("{:<width$}: {}", "release date", RELEASE_DATE, width = max_key_width);
    println!("{:<width$}: {}", "lfy server", mcp_config_endpoint(), width = max_key_width);
    println!("{:<width$}: {}", "device id", id, width = max_key_width);
    Ok(())
}

