use std::sync::OnceLock;

use anyhow::{bail, Result};

static DEVICE_ID: OnceLock<String> = OnceLock::new();

/// 获取当前机器的稳定 `device_id`。
///
/// 约束：
/// - 同一台机器多次运行应保持一致（重装系统不保证）。
/// - 获取失败时返回错误，不返回空值。
pub fn get_device_id() -> Result<String> {
    if let Some(v) = DEVICE_ID.get() {
        return Ok(v.clone());
    }

    let id = get_device_id_uncached()?;
    // 若并发竞争导致 set 失败，则返回已有值（或返回当前 id）。
    let _ = DEVICE_ID.set(id.clone());
    Ok(id)
}

fn get_device_id_uncached() -> Result<String> {
    // macOS
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;

        let output = Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
            .map_err(|e| anyhow::anyhow!("调用 ioreg 获取 device_id 失败: {e}"))?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        parse_macos_ioreg_output_for_uuid(&stdout)
    }

    // Linux
    #[cfg(target_os = "linux")]
    {
        let contents = std::fs::read_to_string("/etc/machine-id")
            .map_err(|e| anyhow::anyhow!("读取 /etc/machine-id 失败: {e}"))?;
        let id = contents.trim();
        if id.is_empty() {
            bail!("machine-id 为空，无法获取 device_id");
        }
        Ok(id.to_string())
    }

    // Windows：HKLM\SOFTWARE\Microsoft\Cryptography\MachineGuid
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::HKEY_LOCAL_MACHINE;
        use winreg::RegKey;

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let key = hklm
            .open_subkey("SOFTWARE\\Microsoft\\Cryptography")
            .map_err(|e| anyhow::anyhow!("打开注册表 Cryptography 失败: {e}"))?;
        let guid: String = key
            .get_value("MachineGuid")
            .map_err(|e| anyhow::anyhow!("读取 MachineGuid 失败: {e}"))?;
        let id = guid.trim();
        if id.is_empty() {
            bail!("MachineGuid 为空，无法获取 device_id");
        }
        Ok(id.to_string())
    }

    // 其它平台
    #[cfg(not(any(
        target_os = "macos",
        target_os = "linux",
        target_os = "windows"
    )))]
    {
        bail!("暂不支持获取 device_id（当前平台）");
    }
}

/// 从 `ioreg -rd1 -c IOPlatformExpertDevice` 输出中解析 `IOPlatformUUID`。
///
/// 返回语义：
/// - 找到则返回 `trim()` 后的 UUID
/// - 找不到则返回 Err（与鉴权链路“不可发送空值”的要求一致）
fn parse_macos_ioreg_output_for_uuid(output: &str) -> Result<String> {
    let start = output
        .find("IOPlatformUUID")
        .ok_or_else(|| anyhow::anyhow!("ioreg 输出中缺少 IOPlatformUUID 字段"))?;

    let tail = &output[start..];

    // ioreg 通常形如：IOPlatformUUID = "A4CE...ADF"
    // 这里先定位 '='，避免先匹配到字段名自身的引号（fixture 下会发生）。
    let eq_pos = tail
        .find('=')
        .ok_or_else(|| anyhow::anyhow!("IOPlatformUUID 字段缺少 '='"))?;
    let after_eq = &tail[eq_pos + 1..];

    let first_quote = after_eq
        .find('"')
        .ok_or_else(|| anyhow::anyhow!("IOPlatformUUID 值未找到引号"))?;
    let after_quote = &after_eq[first_quote + 1..];

    let end_quote = after_quote
        .find('"')
        .ok_or_else(|| anyhow::anyhow!("IOPlatformUUID 值未闭合引号"))?;

    let uuid = after_quote[..end_quote].trim();
    if uuid.is_empty() {
        bail!("IOPlatformUUID 解析结果为空");
    }
    Ok(uuid.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_macos_ioreg_output_for_uuid_ok() {
        let fixture = r#"
{
  "IOPlatformUUID" = "A4CEDE29-306E-56C3-A109-4CD9D2A45ADF"
}
"#;
        let id = parse_macos_ioreg_output_for_uuid(fixture).unwrap();
        assert_eq!(id, "A4CEDE29-306E-56C3-A109-4CD9D2A45ADF");
    }

    #[test]
    fn parse_macos_ioreg_output_for_uuid_missing_err() {
        let fixture = r#"
{
  "something_else" = "x"
}
"#;
        assert!(parse_macos_ioreg_output_for_uuid(fixture).is_err());
    }
}

