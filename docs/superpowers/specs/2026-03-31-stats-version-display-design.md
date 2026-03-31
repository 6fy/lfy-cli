# stats 命令增加版本显示设计

## 概述

在 `lfy-cli stats` 命令输出中增加版本号显示，版本号统一在 `Cargo.toml` 维护。

## 改动设计

### 1. 版本号维护
- **数据源**: `Cargo.toml` 中的 `version` 字段
- **格式**: `26.03.31`（日期风格，无 `v` 前缀）
- **输出时**: 动态拼接 `v` 前缀显示

### 2. stats 命令输出格式
```
version: v26.03.31
device id: A4CEDE29-306E-56C3-A109-4CD9D2A45ADF
```

### 3. 改动文件

| 文件 | 改动内容 |
|------|----------|
| `Cargo.toml` | `version = "0.1.0"` → `version = "26.03.31"` |
| `src/cmd/stats.rs` | 调整输出顺序，输出两行：version 在上，device id 在下 |

### 4. 实现细节

**stats.rs**:
```rust
pub async fn handle_stats_cmd(_matches: &ArgMatches) -> Result<()> {
    let id = device_id::get_device_id()?;
    let version = env!("CARGO_PKG_VERSION");
    println!("version: v{}", version);
    println!("device id: {}", id);
    Ok(())
}
```

## 迭代维护

后续版本迭代时，只需更新 `Cargo.toml` 中的 `version` 字段即可，代码无需改动。
