
# 安装与快速开始

## 环境要求

- Node.js >= 22 (`npm` / `npx`)
- 陆份仪用户 Key / Secret（由陆份仪官方授权后提供）

## 安装

```bash
# 安装 CLI
npm install -g @6fy/cli

# 安装 CLI SKILL（必需）
npx skills add 6fy/lfy-cli -y -g
```

## 快速开始

```bash
# 获取当前机器的 device_id
lfy-cli status

# 登录（由陆份仪官方授权后提供）
lfy-cli login --user-key "your_user_key" --user-secret "your_user_secret"

# 调用客户搜索
lfy-cli customer search '{"keywords":"科技"}'
```

## 升级与卸载

```bash
# 升级 CLI
npm install -g @6fy/cli

# 升级 CLI SKILL
npx skills update 6fy/lfy-cli -y -g

# 刷新工具配置
lfy-cli upgrade

# 卸载 CLI
npm uninstall -g @6fy/cli

# 卸载 CLI SKILL
npx skills remove 6fy/lfy-cli -y -g
```
