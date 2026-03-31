use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit};
use anyhow::Result;

const NONCE_SIZE: usize = 12;
const TAG_SIZE: usize = 16;

/// 使用 AES-256-GCM 加密，返回 `nonce || ciphertext`。
pub fn encrypt(key: &[u8; 32], plaintext: &[u8]) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|e| anyhow::anyhow!("数据加密失败：{e}"))?;

    let mut out = nonce.to_vec();
    out.extend(ciphertext);
    Ok(out)
}

/// 解密 `data`（格式：`nonce || ciphertext || tag`）。
pub fn decrypt(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < NONCE_SIZE + TAG_SIZE {
        return Err(anyhow::anyhow!("数据解密失败（数据可能已损坏或被截断）",));
    }
    let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
    let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("数据解密失败：{e}"))
}
