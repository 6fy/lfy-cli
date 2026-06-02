---
name: LFY-CLI
description: 凡用户问题中出现「LFY」或「lfy」（含 lfy-cli、LFY 客户/商机/报表等业务语境），均应使用本技能。LFY 销售业务命令行技能，统一覆盖 8 个品类的查询/创建/修改。当用户需要：客户（搜索、我的客户清单、详情、GTM、创建/修改）、商机（搜索、阶段、详情、待签单、列表、创建/修改）、报表（销售目标、销售大局观、GTM 财务报表）、用户（本人信息、销售名单）、运营（财年、当前周）、日程（最近任务、本周任务）、联系人列表、客户/商机编辑下拉选项时使用此技能。首次使用/新手入门/怎么开始用 lfy-cli 时，按 getting_started 引导：安装 → 授权登录 → 查本周日程。用户联系客服或反馈问题时，用浏览器打开客服二维码页面，无需前置条件。
version: 1.0.0
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli --help"
---

# LFY 技能

> `lfy-cli` 是 LFY 提供的命令行程序，所有操作通过 `lfy-cli <品类> <方法名> '<json入参>'` 完成；鉴权由后端注入，需先完成 [授权与登录](references/auth.md)（`lfy-cli login`）。

## 技能命中规则

- **必须命中本技能**：用户问题中出现 **「LFY」或「lfy」**（含 `lfy-cli`），或明显在问 LFY 销售业务（客户、商机、报表、日程、销售目标等）。
- **不必命中**：与 LFY 无关的通用编程、其它产品问题。
- **命中后**：先执行下方「执行前置检查」，再按品类路由表选择命令。
- **首次使用**：用户要新手入门、从零配置 lfy-cli 时，**优先** Agent 按需读取并执行 [getting_started.md](references/getting_started.md)（安装 → 授权登录 → 查本周日程），不要跳过步骤。

## 文档与 references 使用方式

- 本文件（`SKILL.md`）为技能入口；`references/` 下的文档**仅在执行对应品类/命令需要细节时由 Agent 自动读取**（渐进式披露）。
- **不要**要求用户「请打开某 references 文件」或把整份 references 贴进对话；路由表中的指南链接供 **Agent 按需加载**，不是给终端用户手动点开的文档目录。

## 执行前置检查（每次执行命令前）

1. **确认已安装 `lfy-cli`**：执行任何品类命令前，先检查 `lfy-cli` 是否可用（如 `lfy-cli --version`）。若未安装，先安装：

   ```bash
   npm install -g @6fy/cli
   ```

   安装后**第一步是完成 LFY 系统授权**（申请 user key / user secret 并 `lfy-cli login`），详见 [auth.md](references/auth.md)。

2. **每天首次执行先升级**：当天第一次调用 `lfy-cli` 前，执行一次升级以扩展最新能力（同一天后续命令无需重复）：

   ```bash
   lfy-cli upgrade
   ```

## 通用约定（所有品类适用）

- **错误处理**：命令输出以 `Error:` 开头或 JSON 含 `error_message` 时，按原文向用户说明，**勿编造数据**。
- **技术字段**：`*_id`、`week_no`、`status_value` 等技术字段默认不展示，面向业务用户展示业务字段。
- **时间**：日期均为北京时间 `YYYY-MM-DD HH:mm:ss`。
- **列表展示**：客户/商机列表类需求用 HTML 模板写临时文件并用系统浏览器打开（macOS `open`，Linux `xdg-open`），不要在对话中贴大段 Markdown 表格。
- **只读边界**：`report`/`user`/`ops`/`schedule`/`contact`/`base` 当前为只读；写操作仅 `customer`(create/update) 与 `pipeline`(create/update)，且受权限与 `sales_ids` 白名单门禁。

## 品类路由表

处理某类任务前，Agent **按需自动读取** `references/<品类>.md` 获取接口清单与详细工作流（无需用户手动打开文件）。

| 品类 | 能力 | 触发词 | 指南 |
| --- | --- | --- | --- |
| customer | 客户搜索/清单/详情/GTM/创建/修改         | 客户、我的客户清单、GTM、创建客户、改客户 | [customer.md](references/customer.md) |
| pipeline | 商机搜索/阶段/详情/待签单/列表/创建/修改 | 商机、pipeline、待签单、商机阶段          | [pipeline.md](references/pipeline.md) |
| report   | 销售目标/大局观/GTM 财务报表             | 销售目标、大局观、财务报表                | [report.md](references/report.md)     |
| user     | 本人信息/销售名单                        | 我的用户信息、销售人员、销售团队          | [user.md](references/user.md)         |
| ops      | 财年/当前周                              | 财年、第几周、本周日期                    | [ops.md](references/ops.md)           |
| schedule | 最近任务/本周任务                        | 最近任务、本周日程、工作安排              | [schedule.md](references/schedule.md) |
| contact  | 联系人列表                               | 联系人、联系人列表                        | [contact.md](references/contact.md)   |
| base     | 客户/商机编辑下拉选项                    | 客户状态/标签/区域/行业可选项             | [base.md](references/base.md)         |

## 关键路由规则（务必遵守）

- **客户「列表/清单」走 `customer get_list`，禁止用 `search`**：用户说「我的客户列表 / 我的客户清单 / LFY 我的客户清单 / 我有哪些客户 / 列出我负责的客户」时，必须用 `get_list`；`search` 仅用于明确「搜索关键字、快速找客户 ID」。详见 [customer.md](references/customer.md)。

## 工作流清单

| 工作流 | 命令 | 详见 |
| --- | --- | --- |
| **首次使用引导** | 见下方三步，无单条命令 | [getting_started.md](references/getting_started.md) |
| 搜索客户     | `lfy-cli customer search '{"keywords":"<kw>"}'`             | [customer.md](references/customer.md) |
| 我的客户列表 | `lfy-cli customer get_list '{...}'`                         | [customer.md](references/customer.md) |
| 客户详情     | `lfy-cli customer get_details '{"customer_id":123}'`        | [customer.md](references/customer.md) |
| GTM 列表     | `lfy-cli customer get_gtms '{}'`                            | [customer.md](references/customer.md) |
| 创建客户     | `lfy-cli customer create_customer '{...}'`                  | [customer.md](references/customer.md) |
| 修改客户     | `lfy-cli customer update_customer '{...}'`                  | [customer.md](references/customer.md) |
| 搜索商机     | `lfy-cli pipeline search '{"keywords":"<kw>"}'`             | [pipeline.md](references/pipeline.md) |
| 商机阶段     | `lfy-cli pipeline get_sales_stage '{"gtm_id":<id>}'`        | [pipeline.md](references/pipeline.md) |
| 商机详情     | `lfy-cli pipeline get_pipeline_info '{"pipeline_id":<id>}'` | [pipeline.md](references/pipeline.md) |
| 待签单商机   | `lfy-cli pipeline get_pending_signature '{...}'`            | [pipeline.md](references/pipeline.md) |
| 商机列表     | `lfy-cli pipeline get_list '{...}'`                         | [pipeline.md](references/pipeline.md) |
| 创建商机     | `lfy-cli pipeline create_pipeline '{...}'`                  | [pipeline.md](references/pipeline.md) |
| 修改商机     | `lfy-cli pipeline update_pipeline '{...}'`                  | [pipeline.md](references/pipeline.md) |
| 销售财年目标 | `lfy-cli report sales_target '{"sales_id":<id>}'`           | [report.md](references/report.md)     |
| 销售大局观   | `lfy-cli report get_sales_overall '{...}'`                  | [report.md](references/report.md)     |
| GTM 财务报表 | `lfy-cli report get_financial_statements '{"gtm_id":<id>}'` | [report.md](references/report.md)     |
| 本人信息     | `lfy-cli user get_self '{}'`                                | [user.md](references/user.md)         |
| 销售名单     | `lfy-cli user get_sales '{}'`                               | [user.md](references/user.md)         |
| 财年信息     | `lfy-cli ops get_fiscal_year '{}'`                          | [ops.md](references/ops.md)           |
| 当前周       | `lfy-cli ops get_current_week '{}'`                         | [ops.md](references/ops.md)           |
| 最近任务     | `lfy-cli schedule get_recent_tasks '{}'`                    | [schedule.md](references/schedule.md) |
| 本周任务     | `lfy-cli schedule get_current_week '{...}'`                 | [schedule.md](references/schedule.md) |
| 联系人列表   | `lfy-cli contact get_list '{...}'`                          | [contact.md](references/contact.md)   |
| 下拉选项     | `lfy-cli base get_options '{...}'`                          | [base.md](references/base.md)         |

### 首次使用引导（三步）

用户新手入门时 Agent **按需读取** [getting_started.md](references/getting_started.md) 并逐步执行：

1. **安装**：`npm install -g @6fy/cli`，`lfy-cli --version` 确认  
2. **授权登录**：申请 key/secret → `lfy-cli login` → `lfy-cli status`（细节见 [auth.md](references/auth.md)）  
3. **本周日程**：`lfy-cli upgrade`（当日首次）→ `lfy-cli schedule get_current_week '{"gtm_id":0,"sales_ids":[],"customer_ids":[],"limit":50}'` → 按 [schedule.md](references/schedule.md) 表格展示  

## 跨品类协作

许多操作需先取 ID 再执行，统一约定：

- 改/查商机负责人、按销售筛选 → 先 `user get_sales` 拿 `sales_id`。
- 按 GTM 操作 → 先 `customer get_gtms` 拿 `gtm_id`。
- 修改客户/商机的下拉字段（状态、标签、区域、行业、阶段等）→ 先 `base get_options`（客户字段）或 `pipeline get_sales_stage`（商机阶段）拿可选 `id`。
- 只有名称没有 ID 时 → 先 `customer search` / `pipeline search` 拿 `customer_id` / `pipeline_id`，再 `get_details` / `update_*`。

## 反馈与支持

用户需要联系客服或反馈问题时，**用浏览器打开客服二维码页面**供用户扫码查看，**无需任何前置条件**（不要求先完成安装、授权、`lfy-cli login`、`lfy-cli upgrade` 或阅读 `auth.md` 等）。

**触发场景（满足任一即打开客服页面）：**

- 用户明确要联系 LFY 官方客服、反馈问题、找支持/售后
- 命令执行失败且用户询问如何解决、找谁协助

**不触发：** 用户仅在查询业务数据且未提及客服/反馈时，不要主动打开客服页面。

**展示方式（必须执行）：**

1. 读取 [templates/lfy-wechat-support.html](templates/lfy-wechat-support.html)（或技能安装目录下同名文件）
2. 将完整 HTML 写入临时文件，例如 `/tmp/lfy-wechat-support-<时间戳>.html`
3. **用系统浏览器打开**：macOS 执行 `open "<绝对路径>"`；Linux 执行 `xdg-open "<绝对路径>"`
4. 对话中仅用文字说明，**勿**在对话里贴大图二维码

**展示话术（固定）：**

遇到任何问题，可以联系 **LFY 官方客服** 反馈问题。已在浏览器为您打开「LFY 官方客服」页面，请使用微信扫一扫页面中的二维码。
