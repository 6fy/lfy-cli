use crate::device_id;

use anyhow::Result;
use clap::ArgMatches;

pub async fn handle_status_cmd(_matches: &ArgMatches) -> Result<()> {
    let id = device_id::get_device_id()?;
    let version = env!("CARGO_PKG_VERSION");
    // 冒号位置固定，key 左对齐，value 从同一列开始显示
    let max_key_width = "device id".len();
    println!("{:<width$}: v{}", "version", version, width = max_key_width);
    println!("{:<width$}: {}", "device id", id, width = max_key_width);
    Ok(())
}

