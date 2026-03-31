mod protocol;

use std::fs;

use anyhow::Result;
pub use protocol::UserCredentials;

use crate::{crypto, fs_util};

pub fn get_credentials() -> Option<UserCredentials> {
    let data = fs::read(credentials_path()).ok()?;
    crypto::try_decrypt_data(&data).ok()
}

pub fn set_credentials(creds: &UserCredentials) -> Result<()> {
    let key = crypto::load_existing_key().unwrap_or_else(|| {
        let k = crypto::generate_random_key();
        tracing::info!("已生成新的加密密钥");
        k
    });

    crypto::save_key(&key)?;

    let encrypted = crypto::encrypt_data(creds, &key)?;

    let path = credentials_path();
    fs_util::atomic_write(&path, &encrypted, Some(0o600))?;

    tracing::info!("凭证已保存到 {}", path.display());
    Ok(())
}

pub fn clear_credentials() {
    let path = credentials_path();
    if path.exists() {
        let _ = fs::remove_file(&path);
        tracing::info!("凭证已删除：{}", path.display());
    }
}

fn credentials_path() -> std::path::PathBuf {
    crate::constants::config_dir().join("credentials.enc")
}
