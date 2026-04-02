
# 安装与快速开始

## 环境要求

- Node.js >= 22 (`npm` / `npx`)
- 陆份仪用户 Key / Secret（可通过陆份仪服务渠道获取）

## 安装

```bash
# 安装 CLI
npm install -g @lfy/cli

# 安装 CLI SKILL（必需）
npx skills add 6fy/lfy-cli -y -g
```

## 快速开始

```bash
# 获取当前机器的 device_id
lfy-cli stats

# 配置 Key / Secret
lfy-cli init --user-key <your_user_key> --user-secret <your_user_secret>

# 调用客户搜索
lfy-cli customer search '{"keywords":"科技"}'
```

## 升级与卸载

```bash
# 升级 CLI
npm install -g @lfy/cli

# 升级 CLI SKILL
npx skills update 6fy/lfy-cli -y -g

# 卸载 CLI
npm uninstall -g @lfy/cli

# 卸载 CLI SKILL
npx skills remove 6fy/lfy-cli -y -g
```

# Agent Skills

---

```bash
# 初始化并刷新 MCP 配置
LFY_MCP_CONFIG_ENDPOINT="http://localhost:16000" cargo run --quiet -- init --refresh

# 重新配置Key Secret
cargo run -- init --user-key <your_user_key> --user-secret <your_user_secret>

# 模糊搜索客户
cargo run -- customer search '{"keywords":"科技"}'


cargo build --release --target aarch64-apple-darwin

npm link
npm install ./packages/darwin-arm64
lfy-cli init
# lfy-cli init --refresh
lfy-cli customer search '{"keywords":"科技"}'

npm unlink @lfy/cli
```

## 身份认证

```bash
# 查看当前device_id
lfy-cli stats
```

# 业务场景

## 客户

```bash
# 关键字查询客户
lfy-cli customer search '{"keywords":"科技"}'


```

**lfy-cli init**

```bash
# 首次配置、修改 Key / Secret
lfy-cli init --user-key <your_user_key> --user-secret <your_user_secret>

# 变更服务器地址（方式一：命令行参数）
lfy-cli init --server-url http://127.0.0.1:16000

# 变更服务器地址（方式二：环境变量）
LFY_MCP_CONFIG_ENDPOINT=http://127.0.0.1:16000 lfy-cli init
```

**优先级：** 环境变量 > 命令行参数 > 默认值（http://localhost:16000）

## 构建 Windows x64 二进制（供 `@lfy/cli-win32-x64`）

在 macOS 上交叉编译 `x86_64-pc-windows-gnu` 需要 **MinGW-w64** 工具链（提供 `x86_64-w64-mingw32-gcc` 等），否则 `aws-lc-sys`、`windows-sys` 等会报找不到编译器 / dlltool。

```bash
# 1. 安装工具链（仅需一次）
brew install mingw-w64
rustup target add x86_64-pc-windows-gnu

# 2. 编译并复制到 packages/win32-x64/bin/lfy-cli.exe
pnpm build
# 等价：bash scripts/publish-win32-x64.sh
```

在 Windows 本机开发可直接：`cargo build --release`，再把 `target/release/lfy-cli.exe` 复制到 `packages/win32-x64/bin/`。

## 发布 npm 包

确保 package.json 中以下版本与 主包版本号一致

```json
"optionalDependencies": {
    "@lfy/cli-darwin-arm64": "0.6.1",
    "@lfy/cli-darwin-x64": "0.6.1",
    "@lfy/cli-linux-x64": "0.6.1",
    "@lfy/cli-win32-x64": "0.6.1"
}
```

把 packages 目录下逐个发布到 npm 仓库，最后发布主包

```bash
npm publish --registry http://nexus.6fenyi.com:8081/repository/npm-hosted-cc/ --access public
```