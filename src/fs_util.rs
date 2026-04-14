use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::Result;

/// 将 `data` 原子写入 `path`（同目录临时文件再 rename）。
pub fn atomic_write(path: &Path, data: &[u8], mode: Option<u32>) -> Result<()> {
    let Some(parent) = path.parent() else {
        anyhow::bail!("无效文件路径：{}", path.display());
    };

    fs::create_dir_all(parent)?;

    let mut tmp = tempfile::NamedTempFile::new_in(parent)?;

    #[cfg(not(unix))]
    let _ = mode;

    #[cfg(unix)]
    if let Some(m) = mode {
        use std::os::unix::fs::PermissionsExt;
        tmp.as_file()
            .set_permissions(fs::Permissions::from_mode(m))?;
    }

    tmp.write_all(data)?;
    tmp.as_file().flush()?;
    tmp.as_file().sync_all()?;

    tmp.persist(path)?;

    Ok(())
}
