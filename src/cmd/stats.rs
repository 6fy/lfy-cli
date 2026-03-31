use crate::device_id;

use anyhow::Result;
use clap::ArgMatches;

pub async fn handle_stats_cmd(_matches: &ArgMatches) -> Result<()> {
    let id = device_id::get_device_id()?;
    let version = env!("CARGO_PKG_VERSION");
    println!("version: v{}", version);
    println!("device id: {}", id);
    Ok(())
}

