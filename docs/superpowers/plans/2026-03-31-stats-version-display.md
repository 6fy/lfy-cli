# stats 命令版本显示实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在 `lfy-cli stats` 命令输出中增加版本号显示

**Architecture:** 版本号统一在 `Cargo.toml` 维护，程序通过 `env!("CARGO_PKG_VERSION")` 宏读取并拼接 `v` 前缀输出

**Tech Stack:** Rust, Clap

---

## 任务 1: 修改 Cargo.toml 版本号

**Files:**
- Modify: `Cargo.toml:3`

- [ ] **Step 1: 修改 Cargo.toml 版本号**

```toml
[package]
name = "lfy-cli"
version = "26.03.31"
edition = "2021"
```

- [ ] **Step 2: 验证构建**

Run: `cargo build`
Expected: BUILD SUCCESS

- [ ] **Step 3: 提交**

```bash
git add Cargo.toml
git commit -m "chore: bump version to 26.03.31"
```

---

## 任务 2: 修改 stats.rs 输出格式

**Files:**
- Modify: `src/cmd/stats.rs:6-11`

- [ ] **Step 1: 修改 stats.rs 输出**

```rust
pub async fn handle_stats_cmd(_matches: &ArgMatches) -> Result<()> {
    let id = device_id::get_device_id()?;
    let version = env!("CARGO_PKG_VERSION");
    println!("version: v{}", version);
    println!("device id: {}", id);
    Ok(())
}
```

- [ ] **Step 2: 验证构建**

Run: `cargo build`
Expected: BUILD SUCCESS

- [ ] **Step 3: 运行测试**

Run: `cargo run -- stats`
Expected:
```
version: v26.03.31
device id: <device_id>
```

- [ ] **Step 4: 提交**

```bash
git add src/cmd/stats.rs
git commit -m "feat: add version display to stats command"
```
