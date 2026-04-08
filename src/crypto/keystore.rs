use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use base64::prelude::*;
use rand::Rng;

use crate::fs_util;

use super::cipher;

pub fn encryption_key_path() -> PathBuf {
    crate::constants::config_dir().join(".encryption_key")
}

fn encode_key(key: &[u8; 32]) -> String {
    BASE64_STANDARD.encode(key)
}

fn decode_key(s: &str) -> Result<[u8; 32]> {
    let bytes = BASE64_STANDARD
        .decode(s)
        .map_err(|e| anyhow::anyhow!("base64 decode error: {e}"))?;
    <[u8; 32]>::try_from(bytes.as_slice())
        .map_err(|_| anyhow::anyhow!("Invalid encryption key length"))
}

pub fn generate_random_key() -> [u8; 32] {
    rand::rng().random()
}

pub fn load_existing_key() -> Option<[u8; 32]> {
    let contents = fs::read_to_string(encryption_key_path()).ok()?;
    decode_key(contents.trim()).ok()
}

pub fn save_key(key: &[u8; 32]) -> Result<()> {
    let b64 = encode_key(key);
    let key_path = encryption_key_path();
    fs_util::atomic_write(&key_path, b64.as_bytes(), Some(0o600))?;
    Ok(())
}

pub fn encrypt_data<T: serde::Serialize + ?Sized>(data: &T, key: &[u8; 32]) -> Result<Vec<u8>> {
    let json =
        serde_json::to_vec(data).map_err(|e| anyhow::anyhow!("JSON serialize error: {e:#}"))?;
    Ok(cipher::encrypt(key, &json)?)
}

pub fn decrypt_data<T: serde::de::DeserializeOwned>(data: &[u8], key: &[u8; 32]) -> Result<T> {
    let decrypted = cipher::decrypt(key, data)?;
    serde_json::from_slice(&decrypted).map_err(|e| anyhow::anyhow!("JSON deserialize error: {e:#}"))
}

pub fn try_decrypt_data<T: serde::de::DeserializeOwned>(data: &[u8]) -> Result<T> {
    let key = load_existing_key().ok_or(anyhow::anyhow!("解密数据失败（未找到有效密钥）"))?;
    decrypt_data(data, &key)
}
