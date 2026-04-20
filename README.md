# lfy-cli

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%3E%3D1.75-orange.svg)](https://www.rust-lang.org/)


## 功能范围

覆盖LFY核心业务品类：

| 命令/品类  | 功能范围                              |
| ---------- | ------------------------------------- |
| `login`    | 登录LFY官方系统，完成账号绑定      |
| `status`   | 查看状态                              |
| `upgrade`  | 升级工具，为智能体扩展能力            |
| `customer` | 客户 - 提供销售核心场景部分可用性查询 |
| `pipeline` | 商机 - 提供销售核心场景部分可用性查询 |
| `user`     | 用户 - 提供用户信息/销售人员查询      |
| `ops`      | 企基 - 提供企业运营基础数据查询       |
| `schedule` | 日程 - 提供日历中工作任务相关查询     |
| `report`   | 报表 - 提供部分报表数据查询           |

以上功能仍在陆续完善中，欢迎提交 PR 或 Issue 提供建议。

## 快速开始

### 环境要求

- Node.js >= 22 (`npm` / `npx`)
- LFY用户 Key / Secret（由LFY官方授权后提供）

### 安装 & 使用

```bash
# 1. 安装 CLI
npm install -g @6fy/cli

# 2. 安装 CLI Skill（必需）
npx skills add 6fy/lfy-cli -y -g

# 3. 📌 国内用户安装 CLI Skill（可选）
npx skills add https://gitee.com/lfy-team/lfy-cli.git -y -g

# 4. 获取当前机器的 device_id(联系LFY官方授权)
lfy-cli status

# 5. 登录（由LFY官方授权后提供），仅需一次
lfy-cli login --user-key "your_user_key" --user-secret "your_user_secret"

# 6. 搜索客户
lfy-cli customer search '{"keywords":"科技"}'
```

> 有关升级与卸载的详细说明，请参见 [升级与卸载.md](docs/升级与卸载.md)。

## Agent Skills

🤖 支持的 Skills 使用说明，请参阅 [Skills 文档](docs/skills.md)。

## 许可证

本项目基于 [MIT 许可证](./LICENSE) 开源。