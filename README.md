


## 功能范围

| 命令       | 功能范围                              |
| ---------- | ------------------------------------- |
| `login`    | 登录陆份仪官方系统，完成账号绑定      |
| `status`   | 查看状态                              |
| `upgrade`  | 升级工具，为智能体扩展能力            |
| `customer` | 客户 - 提供销售核心场景部分可用性查询 |
| `pipeline` | 商机 - 提供销售核心场景部分可用性查询 |
| `user`     | 用户 - 提供用户信息/销售人员查询      |
| `ops`      | 企基 - 提供企业运营基础数据查询       |
| `schedule` | 日程 - 提供日历中工作任务相关查询     |

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