# Server URL 配置实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 支持 `lfy-cli init --server-url <url>` 命令，将服务器地址持久化保存到配置文件，并实现优先级：环境变量 > 配置文件 > 默认值。

**Architecture:** 新增 `settings` 模块管理配置持久化，修改 `constants.rs` 中的 `mcp_config_endpoint()` 函数实现优先级逻辑，修改 `init.rs` 添加 `--server-url` 参数。

**Tech Stack:** Rust, serde, 现有 config 目录结构

---

## Task 1: 创建 settings 模块

**Files:**
- Create: `src/settings.rs`

- [ ] **Step 1: 创建 settings.rs 文件**

```rust
use serde::{Deserialize, Serialize};

use crate::constants::config_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server_url: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { server_url: None }
    }
}

pub fn settings_path() -> std::path::PathBuf {
    config_dir().join("settings.json")
}

pub fn load_settings() -> Settings {
    let path = settings_path();
    if let Ok(data) = std::fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Settings::default()
    }
}

pub fn save_settings(settings: &Settings) -> anyhow::Result<()> {
    use std::io::Write;
    let path = settings_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::File::create(&path)?;
    write!(file, "{}", serde_json::to_string_pretty(settings)?)?;
    Ok(())
}
```

- [ ] **Step 2: 验证文件编译通过**

Run: `cargo check`
Expected: 无错误

- [ ] **Step 3: Commit**

```bash
git add src/settings.rs
git commit -m "feat: add settings module for config persistence"
```

---

## Task 2: 修改 constants.rs 实现优先级逻辑

**Files:**
- Modify: `src/constants.rs:1-36`

- [ ] **Step 1: 添加 load_server_url_from_config 函数**

在 `constants.rs` 文件末尾添加：

```rust
fn load_server_url_from_config() -> Option<String> {
    // 延迟加载 settings 模块，避免循环依赖
    if let Ok(settings) = crate::settings::load_settings() {
        settings.server_url.filter(|s| !s.is_empty())
    } else {
        None
    }
}

pub fn mcp_config_endpoint() -> String {
    // 1. 环境变量（最高优先）
    if let Ok(url) = std::env::var(env::MCP_CONFIG_ENDPOINT) {
        if !url.is_empty() {
            return url;
        }
    }

    // 2. 配置文件
    if let Some(url) = load_server_url_from_config() {
        return url;
    }

    // 3. 默认值
    DEFAULT_MCP_CONFIG_ENDPOINT.to_string()
}
```

- [ ] **Step 2: 验证编译**

Run: `cargo check`
Expected: 无错误

- [ ] **Step 3: Commit**

```bash
git add src/constants.rs
git commit -m "feat: implement server url priority: env > config > default"
```

---

## Task 3: 修改 init.rs 添加 --server-url 参数

**Files:**
- Modify: `src/cmd/init.rs:1-72`

- [ ] **Step 1: 添加 server_url 字段到 InitArgs**

修改 `InitArgs` 结构体：

```rust
#[derive(Args)]
pub struct InitArgs {
    #[arg(long, help = "仅刷新 MCP 配置缓存（不更新 User Key / User Secret）")]
    refresh: bool,

    #[arg(long, help = "User Key（非交互时可传入）")]
    user_key: Option<String>,

    #[arg(long, help = "User Secret（非交互时可传入，注意 shell 历史）")]
    user_secret: Option<String>,

    #[arg(long, help = "MCP Server 地址")]
    server_url: Option<String>,
}
```

- [ ] **Step 2: 添加 server_url 保存逻辑**

在 `handle_init_cmd` 函数开头添加：

```rust
// 处理 server_url 配置
if let Some(url) = &args.server_url {
    let mut settings = crate::settings::load_settings();
    settings.server_url = Some(url.clone());
    crate::settings::save_settings(&settings)?;
    println!("Server 地址已保存: {}", url);
}
```

- [ ] **Step 3: 更新 main.rs 导出 settings 模块**

修改 `src/main.rs`，在 `mod` 声明区域添加：

```rust
mod settings;
```

- [ ] **Step 4: 验证编译**

Run: `cargo check`
Expected: 无错误

- [ ] **Step 5: Commit**

```bash
git add src/cmd/init.rs src/main.rs
git commit -m "feat: add --server-url option to init command"
```

---

## Task 4: 更新 README.md 文档

**Files:**
- Modify: `README.md:58-61`

- [ ] **Step 1: 更新变更服务器地址部分**

将：

```markdown
# 变更服务器地址
lfy-cli init --server-url http://127.0.0.1:16000
```

更新为：

```markdown
# 变更服务器地址

## 方式一：命令行参数（推荐）
lfy-cli init --server-url http://127.0.0.1:16000

## 方式二：环境变量
LFY_MCP_CONFIG_ENDPOINT=http://127.0.0.1:16000 lfy-cli init

## 优先级
环境变量 > 命令行参数 > 默认值（http://localhost:16000）
```

- [ ] **Step 2: Commit**

```bash
git add README.md
git commit -m "docs: update README with server-url config options"
```

---

## Task 5: 手动测试

- [ ] **Step 1: 测试默认配置**

Run: `cargo run -- init --help`
Expected: 看到 `--server-url` 选项

- [ ] **Step 2: 测试配置文件保存**

Run: `cargo run -- init --server-url http://test.local:16000`
Expected: 输出 "Server 地址已保存: http://test.local:16000"

- [ ] **Step 3: 验证配置文件**

Run: `cat ~/.config/lfy/settings.json`
Expected: `{"server_url":"http://test.local:16000"}`

- [ ] **Step 4: 测试优先级（环境变量覆盖）**

Run: `LFY_MCP_CONFIG_ENDPOINT=http://env.local:16000 cargo run -- init --server-url http://other.local:16000`
Expected: 使用环境变量的值（因为优先级最高）

---

## 实施完成

**总结改动：**
- 新增 `src/settings.rs` - 配置持久化模块
- 修改 `src/constants.rs` - 实现优先级逻辑
- 修改 `src/cmd/init.rs` - 添加 `--server-url` 参数
- 修改 `src/main.rs` - 导出 settings 模块
- 修改 `README.md` - 更新文档
