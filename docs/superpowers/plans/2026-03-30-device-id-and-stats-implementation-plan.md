# Device ID Binding & `lfy-cli stats` Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 为 `lfy-cli` 增加 `device_id` 双处校验（`init` 的 `mcp/auth/validate` + `customer/*` 的 `arguments.auth` 注入）并新增 `lfy-cli stats` 命令输出当前机器 `device_id`。

**Architecture:** 新增 `src/device_id.rs` 作为统一入口，负责按平台读取稳定机器标识并在进程内缓存。鉴权链路在 `src/mcp/config.rs`（validate）与 `src/cmd/call.rs`（注入 auth）按约定加入 `device_id`。命令扩展通过 `src/cmd/stats.rs` 和 `src/main.rs` Clap 子命令完成。

**Tech Stack:** Rust（tokio/reqwest/serde/anyhow/clap），macOS `ioreg`，Linux 读取 `/etc/machine-id`。

---

### Task 1: 实现 `device_id` 获取模块（含解析与缓存）

**Files:**
- Create: `src/device_id.rs`
- Modify: `src/main.rs`（增加 `mod device_id;`，确保后续在 `src/cmd/init.rs` / `src/cmd/call.rs` / `src/cmd/stats.rs` 中可用）

- [ ] Step 1: 写失败的单测（macOS ioreg 输出解析）

  1) 在 `src/device_id.rs` 中抽象 `parse_macos_ioreg_output_for_uuid(output: &str) -> Result<String>`。
  2) 直接在测试里内联 fixture 字符串（不要依赖本地终端快照文件），fixture 至少包含一行：
     - `IOPlatformUUID = "A4CEDE29-306E-56C3-A109-4CD9D2A45ADF"`
  3) 断言解析得到的 `device_id` 等于 `A4CEDE29-306E-56C3-A109-4CD9D2A45ADF`。
  4) 补一个失败断言：当 fixture 不包含 `IOPlatformUUID` 时必须 `Err`（解析语义固定）。

- [ ] Step 2: 跑测试，确保失败（因为当前模块还不存在/解析函数未实现）

Run: `cargo test -q`
Expected: 失败（编译失败或测试断言失败）

- [ ] Step 3: 最小实现（通过单测）

  - macOS：调用 `ioreg -rd1 -c IOPlatformExpertDevice`，将输出交给 `parse_macos_ioreg_output_for_uuid`。
    - 提取规则（不引入新的 crate，避免改 Cargo.lock）：
      - 在输出中找到子串 `IOPlatformUUID`
      - 从该位置向后找到第一个 `"`，取后续到下一个 `"` 之间的内容
      - 结果 `trim()` 后返回（通常本身已无多余空白）
    - 若取不到：返回 `Err`（与 Step 1 的失败断言一致）
  - Linux：读取 `/etc/machine-id` 并 `trim()`，若空则返回错误
  - Windows：暂不支持：返回错误 `暂不支持获取 device_id`
  - 进程缓存：使用 `std::sync::OnceLock<String>` 缓存 `device_id`，避免重复读取

- [ ] Step 4: 跑测试确认通过

Run: `cargo test -q`
Expected: PASS

- [ ] Step 5:（不提交 git）仅本地编译验证

Run: `cargo test -q && cargo build -q`

---

### Task 2: 在 `mcp/auth/validate` 请求中注入 `device_id`

**Files:**
- Modify: `src/mcp/config.rs`
- Modify: `src/cmd/init.rs`

- [ ] Step 1: 修改函数签名并更新调用点

  - `mcp::config::validate_user_credentials_on_server` 增加 `device_id: &str` 参数。
  - `mcp::config::validate_user_credentials` 增加 `device_id` 参数，并透传到 on_server。
  - `src/cmd/init.rs` 在调用 validate 前获取 `device_id`：`let device_id = device_id::get_device_id()?;`

- [ ] Step 2: 修改 JSON-RPC body

  - 在 `params` 中新增 `device_id`：
    - `{"user_key":..., "user_secret":..., "device_id":...}`

- [ ] Step 3: 约束错误解析不变（保证 spec 失败语义）

  - 在 `src/mcp/config.rs` 中：`validate_user_credentials_on_server` 对 JSON-RPC 响应的解析逻辑（`result.ok` / `result.error` / `error.message`）**保持不变**。
  - 本任务只修改请求 `params` 与函数签名，不改动现有的错误分支结构，避免鉴权失败文案/错误结构偏离 spec。

- [ ] Step 3: 跑编译与测试

Run: `cargo test -q && cargo build -q`
Expected: PASS

---

### Task 3: 在 `customer/*` 工具调用注入 `arguments.auth.device_id`

**Files:**
- Modify: `src/cmd/call.rs`

- [ ] Step 1: 修改 auth 注入结构

  - 在 `src/cmd/call.rs` 的 `if category_name == "customer" && full_method != "customer/is_available"` 分支中，
    - 读取本地 `user_key/user_secret`（现有逻辑不变）
    - 获取 `device_id`
    - 仅当 `obj` 中不存在 `auth` 字段时才注入：
      - 保持现有 `if !obj.contains_key("auth") { obj.insert("auth", ...) }` 逻辑
    - 注入的 `auth` 内容包含三项：
      - `{"user_key":..., "user_secret":..., "device_id":...}`
    - 若获取 `device_id` 失败：必须使用 `?` 直接返回错误，确保不会调用 `json_rpc::send(...)`。

- [ ] Step 2: 保持兼容

  - 当参数对象中已存在 `auth` 字段：沿用当前行为（不覆盖）
  - `customer/is_available` 仍保持不注入 auth

- [ ] Step 3: 跑编译与测试

Run: `cargo test -q && cargo build -q`

---

### Task 4: 新增 `lfy-cli stats`（单行输出当前 `device_id`）

**Files:**
- Create: `src/cmd/stats.rs`
- Modify: `src/cmd/mod.rs`
- Modify: `src/main.rs`

- [ ] Step 1: 实现 `src/cmd/stats.rs`

  - 子命令无参数
  - 获取 `device_id` 并 `println!("{}", device_id)`
  - 获取失败时返回错误（非 0）
  - 导出函数签名：`pub async fn handle_stats_cmd(_matches: &ArgMatches) -> Result<()>`（若与现有项目风格不一致，按现有 `init/call` 写法等价实现）

- [ ] Step 2: Clap 注册

  - 在 `src/main.rs` 的 Clap 子命令构建处显式新增：
    - `cmd = cmd.subcommand(Command::new("stats").about("查看当前机器的 device_id"))`
  - 在 `match matches.subcommand()` 中新增分支，且必须放在 catch-all category 分支之前：
    - `Some(("stats", matches)) => cmd::stats::handle_stats_cmd(matches).await`
  - 保证 `stats` 不会被 `cmd::call::handle_call_cmd("stats", ...)` 吃掉。

- [ ] Step 3: 修改 `src/cmd/mod.rs`

  - 添加：`pub mod stats;`

- [ ] Step 4: 编译与手动验证

Run: `cargo build -q`
Manual:
- `./target/debug/lfy-cli stats` 输出单行非空

---

### Task 5: 本地验证鉴权链路（不依赖 git 提交）

Run:
- `./target/debug/lfy-cli init`（如果你已有 key/secret）
- `./target/debug/lfy-cli customer search '{"keywords":"科技"}'`

Expected:
- 当后端允许该 device_id：请求成功
- 当 device_id 不匹配：服务端拒绝并提示错误（透传策略与当前行为保持一致）

