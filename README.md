
# 安装与快速开始

## 环境要求

- Node.js >= 22 (`npm` / `npx`)
- 陆份仪用户 Key / Secret（可通过陆份仪服务渠道获取）

## 安装

```bash
# 安装 CLI
npm install -g @lfy/cli

# 安装 CLI SKILL（必需）
npx skills add lfy-cli -y -g
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