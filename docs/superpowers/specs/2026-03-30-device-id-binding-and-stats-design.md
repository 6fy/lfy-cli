## 设备绑定校验与 `lfy-cli stats` 设计

### 背景
你已在后端表 `captain_ai.agent_cli_user` 新增字段 `device_id`，用于将 `user_key/user_secret` 的绑定进一步绑定到“同一台机器”。同时你期望在每次校验 `user_key/user_secret` 时都额外校验 `device_id`，并要求 `lfy-cli` 支持一个新命令 `lfy-cli stats` 用于查看当前机器的 `device_id`。

### 目标
1. 在 `lfy-cli init` 的鉴权校验（调用 `mcp/auth/validate`）中，除 `user_key/user_secret` 外额外携带 `device_id`。
2. 在所有需要鉴权的业务调用中（当前为 `customer/*`，排除 `customer/is_available`），在 `arguments.auth` 注入 `device_id`，让服务端可进行二次校验。
3. 新增 `lfy-cli stats`：单行输出当前机器的 `device_id`（值每次读取保持一致）。

### 非目标
1. 不在本地持久化存储 `device_id`（避免与硬件/系统状态不一致导致的漂移；只在运行时计算并缓存到进程内）。
2. 不引入复杂指纹多源融合方案（如 MAC/CPU 混合），以降低漂移风险。

### 关键决策
1. `device_id` 取值策略：使用“同一台机器稳定的系统标识”，并在进程内缓存；不保证跨重装系统（与你选择的 A 一致）。
2. 字段名：对齐后端约定，统一使用 `device_id`（下划线）。
3. 稳定性：同一机器、每次取值一致（重装系统可能变化）。

### `device_id` 获取策略（方案 1：OS 稳定机器标识）
实现一个统一入口 `get_device_id()`：
1. macOS：读取 `IOPlatformUUID`（`IOPlatformExpertDevice` 的平台 UUID），解析成字符串作为 `device_id`。
2. Linux：读取 `/etc/machine-id`，去除首尾空白后作为 `device_id`。
3. Windows：若暂不提供可靠实现，则返回明确错误提示“暂不支持获取 device_id”，避免把空值发送给服务端导致鉴权绕过或误拒绝。

进程内缓存：
- 使用 `OnceLock`/`lazy` 方式缓存 `device_id`，避免每次命令/每次请求都反复调用系统读取。

### 鉴权与业务调用的数据流变更
#### 1) `lfy-cli init` 鉴权校验
现状：
- `init` 调用 `mcp/auth/validate` 校验 `user_key/user_secret`。

变更：
- 在 `mcp/auth/validate` 的 `params` 中新增字段：
  - `device_id: <当前设备ID>`

失败语义：
- 当服务端判定 `user_key/user_secret/device_id` 不匹配，应保持当前行为：将服务端返回的 401 提示错误信息透传给用户。

#### 2) `customer/*` 业务调用注入 auth
现状：
- `src/cmd/call.rs` 对 `customer/*`（排除 `customer/is_available`）注入 `arguments.auth = { user_key, user_secret }`。

变更：
- 将注入扩展为：
  - `arguments.auth = { user_key, user_secret, device_id }`

保持兼容：
- `customer/is_available` 继续不注入 auth（除非后端另有要求）。

### 新命令：`lfy-cli stats`
行为：
- 子命令形式：`lfy-cli stats`
- 输出：单行文本，内容为 `<device_id>`。

实现入口（CLI 侧）：
- 在 `src/main.rs` 的 Clap 子命令注册中新增 `stats`，并派发到 `src/cmd/stats.rs`。

错误处理：
- 如果无法获取 `device_id`（例如 Windows 暂不支持、权限不足），返回非 0 退出码并打印明确错误原因。

### 需要修改的代码点（仅列出位置，后续进入实现计划）
1. 新增模块：`src/device_id.rs`
2. 鉴权请求：`src/mcp/config.rs`
   - `validate_user_credentials` / `validate_user_credentials_on_server` 增加 `device_id` 参数，并在 JSON-RPC body 中携带。
3. 初始化流程：`src/cmd/init.rs`
   - 在调用鉴权校验前获取并传入 `device_id`。
4. 业务调用注入：`src/cmd/call.rs`
   - 注入 `arguments.auth` 时追加 `device_id`。
5. 新命令：`src/cmd/stats.rs` + `src/main.rs` Clap 注册 + `src/cmd/mod.rs` 导出

### 测试与验证（实现阶段执行）
1. 手动验证：
   - `lfy-cli stats` 输出非空且稳定（连续两次运行结果一致）。
   - `lfy-cli init`：当 device_id 正确绑定时可通过；device_id 不匹配时被服务端拒绝。
   - `lfy-cli customer search '{"keywords":"科技"}'`：正常返回；当 device_id 不匹配时失败并提示错误。
2. 单元测试（可选，视实现细节）：
   - macOS 解析逻辑的输出样例解析。
   - Linux `/etc/machine-id` 读取与 trim 逻辑。

### 风险与对策
1. 风险：系统标识读取权限/命令不可用导致无法取值。
   - 对策：失败时不发送空值；直接返回错误，提示用户环境/平台问题。
2. 风险：服务端对字段名或校验语义存在差异。
   - 对策：严格按 `device_id` 字段名注入，并在实现前对齐后端期望的 JSON 结构（本 spec 以当前 CLI 注入位置为准）。

