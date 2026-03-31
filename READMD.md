
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


# 业务场景

## 客户

```bash
# 关键字查询客户
lfy-cli customer search '{"keywords":"科技"}'


```