---
name: LFY-CLI
description: 凡问题可用本技能 8 品类 lfy-cli 命令回答（含「我是哪个企业」等身份/组织、客户商机报表日程等，即使用户未提 LFY），或出现 LFY/lfy、明显 LFY 销售业务语境，均必须使用本技能。LFY 销售业务命令行技能，统一覆盖 8 个品类的查询/创建/修改。当用户需要：客户（搜索、我的客户清单、详情、GTM、创建/修改、添加/查看跟进记录）、商机（搜索、阶段、详情、待签单、列表、创建/修改、添加/查看跟进记录）、报表（销售目标、销售大局观、GTM 财务报表）、用户（本人信息、销售名单）、运营（财年、当前周）、日程（最近任务、本周任务、指定日期区间任务、创建任务）、联系人列表、客户/商机编辑下拉选项时使用此技能。首次使用/新手入门/怎么开始用 lfy-cli 时，按 getting_started 引导：安装 → 授权登录 → 查本周日程。用户联系客服或反馈问题时，用浏览器打开客服二维码页面，无需前置条件。
version: 1.0.6
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli --help"
---

# LFY 技能

> `lfy-cli` 是 LFY 提供的命令行程序，所有操作通过 `lfy-cli <品类> <方法名> '<json入参>'` 完成；鉴权由后端注入，需先完成 [授权与登录](references/auth.md)（`lfy-cli login`）。

## LFY-CLI 帮助手册

完整使用说明、常见问题与操作指引见飞书文档：**[LFY-CLI 帮助手册](https://ccccccccc.feishu.cn/docx/Z1wAdj7Ipo34GvxN2hNcfQ2hnbg)**。

- 用户询问「帮助」「手册」「文档」「怎么用 LFY-CLI」「完整教程」时，除按本技能执行命令外，**应提供上述链接**供用户查阅。
- 本技能 `references/` 侧重命令参数与 Agent 执行细节；更全面的图文说明以飞书手册为准。

## 技能命中规则

- **能力命中（优先）**：只要用户问题能用本技能 **8 品类、下方工作流清单中任一** `lfy-cli` 命令回答（即使用户**未**说 LFY/lfy），**必须**使用本技能。
- **关键词命中**：用户问题中出现 **「LFY」或「lfy」**（含 `lfy-cli`），或明显在问 LFY 销售业务（客户、商机、报表、日程、销售目标等），**必须**使用本技能。
- **二者满足其一即命中**；禁止用通用知识编造客户/商机/报表/日程等数据。
- **不必命中**：与 LFY 销售数据无关的通用编程、其它产品、闲聊；明确指向**其它系统**（如 Salesforce、钉钉人事）且无法映射到本技能命令时。
- **命中后**：先执行下方「执行前置检查」，再按品类路由表选择命令。
- **首次使用**：用户要新手入门、从零配置 lfy-cli 时，**优先** Agent 按需读取并执行 [getting_started.md](references/getting_started.md)（安装 → 授权登录 → 查本周日程），不要跳过步骤。

## 强制命中场景（示例）

以下说法**常不含 LFY 字眼**，但只要意图落在对应能力，即适用上文「能力命中」，**禁止**跳过本技能：

| 品类                  | 示例说法（节选）                                                      | 命令                                                                                   |
| --------------------- | --------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| **user（身份/组织）** | 我是哪个企业、我是谁、我属于哪家公司、当前登录用户、我的组织/企业名称 | `user get_self`                                                                        |
| user                  | 销售有哪些、销售团队、业务员名单                                      | `user get_sales`                                                                       |
| ops                   | 第几周、本周日期、当前财年                                            | `ops get_current_week` / `ops get_fiscal_year`                                         |
| schedule              | 本周安排、最近有什么任务、指定日期任务、创建任务/日程                 | `schedule get_current_week` / `get_recent_tasks` / `get_tasks_anytime` / `create_task` |
| customer              | 我的客户清单、搜客户、客户详情、GTM 列表、记跟进、写备注、记录沟通、查看跟进记录 | `customer add_follow_up` / `get_details`；详见 [customer.md](references/customer.md) |
| pipeline              | 搜商机、待签单、商机列表/详情/阶段、记跟进、写备注、记录沟通、查看商机跟进       | `pipeline add_follow_up` / `get_pipeline_info`；详见 [pipeline.md](references/pipeline.md) |
| report                | 销售目标、销售大局观、财务报表                                        | 见 [report.md](references/report.md)                                                   |
| contact / base        | 联系人列表、客户状态下拉选项等                                        | 见 [contact.md](references/contact.md)、[base.md](references/base.md)                  |

更全触发词见「品类路由表」与各 `references/<品类>.md`。

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
- **结构化报错自愈**（命中即按动作处理，不要把同一错误原样重试）：
  - `missing <字段>`（如 `missing start_date`）→ 补齐该必填字段或向用户追问后再调用
  - `missing gtm_id` / `缺少 gtm_id`（含 `report get_financial_statements` 等必填 gtm_id 命令）→ **先 `customer get_gtms` 取 `gtm_id`**（名称匹配不到或多义时向用户确认 GTM）再调用；**禁止**用 `{}` 原样重试
  - `日期区间不能超过60天` → 缩短区间或分段多次调用
  - `结束日期不能早于开始日期` → 交换/修正日期后重试
  - 写操作（`customer`/`pipeline` 的 create/update）返回 `参数错误` → 自检是否「把名称当 id」或「用错键名」（如改客户状态误用 `customer_status` 应为 `status_id`）→ 先 `base get_options`（客户字段）或 `pipeline get_sales_stage`（商机阶段）换成 id 再**重试一次**，禁止原样重试
  - `add_follow_up` 返回 content 相关参数错误（如 `content 不能为空`）→ 检查是否未包 `<p></p>`，自动包裹后**重试一次**
  - `add_follow_up` 返回 `客户不存在` / `商机不存在` → 先 `customer search` / `pipeline search` 确认 ID 再重试
  - 其它 `Error:` → 原文转述，不编造
- **技术字段**：`*_id`、`week_no`、`status_value` 等技术字段默认不展示，面向业务用户展示业务字段。
- **时间**：日期均为北京时间 `YYYY-MM-DD HH:mm:ss`。
- **列表展示**：客户/商机列表类需求用 HTML 模板写临时文件并用系统浏览器打开（macOS `open`，Linux `xdg-open`），不要在对话中贴大段 Markdown 表格。

## 品类路由表

处理某类任务前，Agent **按需自动读取** `references/<品类>.md` 获取接口清单与详细工作流（无需用户手动打开文件）。

| 品类     | 能力                                     | 触发词                                               | 指南                                  |
| -------- | ---------------------------------------- | ---------------------------------------------------- | ------------------------------------- |
| customer | 客户搜索/清单/详情/GTM/创建/修改/添加·查看跟进 | 客户、我的客户清单、GTM、创建客户、改客户、添加跟进、写备注、跟进记录、记录沟通 | [customer.md](references/customer.md) |
| pipeline | 商机搜索/阶段/详情/待签单/列表/创建/修改/添加·查看跟进 | 商机、pipeline、待签单、商机阶段、添加跟进、写备注、跟进记录、记录沟通 | [pipeline.md](references/pipeline.md) |
| report   | 销售目标/大局观/GTM 财务报表             | 销售目标、大局观、财务报表                           | [report.md](references/report.md)     |
| user     | 本人信息/销售名单                        | 我的用户信息、销售人员、销售团队                     | [user.md](references/user.md)         |
| ops      | 财年/当前周                              | 财年、第几周、本周日期                               | [ops.md](references/ops.md)           |
| schedule | 最近任务/本周任务/区间任务/创建任务      | 最近任务、本周日程、指定日期任务、创建任务、工作安排 | [schedule.md](references/schedule.md) |
| contact  | 联系人列表                               | 联系人、联系人列表                                   | [contact.md](references/contact.md)   |
| base     | 客户/商机编辑下拉选项                    | 客户状态/标签/区域/行业可选项                        | [base.md](references/base.md)         |

## 关键路由规则（务必遵守）

- **身份/组织必走 `user get_self`**：用户问企业/组织/公司/我是谁/当前账号等（含「我是哪个企业」）→ **禁止**猜测或编造 → 执行 `lfy-cli user get_self '{}'` → 面向用户展示 **`{org_name} - {user_name}`**（不展示 `user_id`）→ 失败按 `Error:` 原文说明，必要时引导 `lfy-cli login`（见 [auth.md](references/auth.md)）。
- **客户「列表/清单」走 `customer get_list`，禁止用 `search`**：用户说「我的客户列表 / 我的客户清单 / LFY 我的客户清单 / 我有哪些客户 / 列出我负责的客户」时，必须用 `get_list`；`search` 仅用于明确「搜索关键字、快速找客户 ID」。详见 [customer.md](references/customer.md)。
- **schedule 写能力边界**：用户要「新建/添加/创建一个任务或日程」→ `schedule create_task`；用户要「改/删/完成/取消/延期 **已有** `task_id` 的任务」→ 说明 CLI 不支持，引导 Web 详情页 `https://app.6fenyi.com/tasks/{task_id}`；**禁止**因「不支持修改已有任务」而拒绝 `create_task`。详见 [schedule.md](references/schedule.md)。
- **schedule 查询命令选择（按顺序匹配，命中即停）**：
  1. 问句**无明确日期/区间** → `get_recent_tasks`（参数固定 `{}`，最稳）
  2. 问句是「本周/这周」 → `get_current_week`
  3. 问句含**明确起止日期**或「上个月 / 某季度 / X 月到 Y 月」 → `get_tasks_anytime`
  - **硬规则**：无法同时确定 `start_date` 与 `end_date` 时，**禁止**调用 `get_tasks_anytime`（会报 `missing start_date`），改用 `get_recent_tasks` 或先向用户追问；**不要**用 `{}` 试探带必填参的命令。
- **必填 `gtm_id` 命令禁止 `{}` 试探**：`report get_financial_statements`（以及 `pipeline get_sales_stage`、`customer create_customer`、`pipeline create_pipeline` 等 `gtm_id` 必填命令）在拿到 `gtm_id` 前，**禁止**用 `{}` 调用（会报 `缺少 gtm_id 参数`）；必须先 `customer get_gtms` 取 `gtm_id`，或向用户确认 GTM 后再调。
- **跟进记录路由（按顺序匹配，命中即停）**：
  1. 用户要**查看/列出**跟进历史 → 客户：`get_details` 展示 `previous_followup_records`；商机：`get_pipeline_info` 展示 `previous_followup_records`
  2. 用户要**记一条/添加/写备注/记录沟通**且对象为客户 → `customer add_follow_up`；**禁止** `update_customer`（无备注字段）；**禁止** `schedule create_task`
  3. 同上且对象为商机 → `pipeline add_follow_up`；**禁止** `update_pipeline`；**禁止** `schedule create_task`
  4. 用户要**带截止日的提醒/任务**（如「6 月 10 号提醒我跟进客户 A」）且无「记到客户/商机时间线」语义 → `schedule create_task`（见 schedule 边界）
  - **硬规则**：`content` 须以 `<p></p>` 包裹；用户给纯文本时自动包一层再调用；无 `customer_id` / `pipeline_id` 时先 `search`；「备注」在 LFY CLI 语境下指跟进记录（`add_follow_up`），不是主档字段修改。详见 [customer.md](references/customer.md)、[pipeline.md](references/pipeline.md)。

## 工作流清单

| 工作流           | 命令                                                                                                                                                | 详见                                                |
| ---------------- | --------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| **首次使用引导** | 见下方三步，无单条命令                                                                                                                              | [getting_started.md](references/getting_started.md) |
| 搜索客户         | `lfy-cli customer search '{"keywords":"<kw>"}'`                                                                                                     | [customer.md](references/customer.md)               |
| 我的客户列表     | `lfy-cli customer get_list '{...}'`                                                                                                                 | [customer.md](references/customer.md)               |
| 客户详情         | `lfy-cli customer get_details '{"customer_id":123}'`                                                                                                | [customer.md](references/customer.md)               |
| GTM 列表         | `lfy-cli customer get_gtms '{}'`                                                                                                                    | [customer.md](references/customer.md)               |
| 创建客户         | `lfy-cli customer create_customer '{...}'`                                                                                                          | [customer.md](references/customer.md)               |
| 修改客户         | `lfy-cli customer update_customer '{...}'`                                                                                                          | [customer.md](references/customer.md)               |
| 添加客户跟进     | `lfy-cli customer add_follow_up '{"customer_id":123,"content":"<p>...</p>"}'`                                                                       | [customer.md](references/customer.md)               |
| 搜索商机         | `lfy-cli pipeline search '{"keywords":"<kw>"}'`                                                                                                     | [pipeline.md](references/pipeline.md)               |
| 商机阶段         | `lfy-cli pipeline get_sales_stage '{"gtm_id":<id>}'`                                                                                                | [pipeline.md](references/pipeline.md)               |
| 商机详情         | `lfy-cli pipeline get_pipeline_info '{"pipeline_id":<id>}'`                                                                                         | [pipeline.md](references/pipeline.md)               |
| 待签单商机       | `lfy-cli pipeline get_pending_signature '{...}'`                                                                                                    | [pipeline.md](references/pipeline.md)               |
| 商机列表         | `lfy-cli pipeline get_list '{...}'`                                                                                                                 | [pipeline.md](references/pipeline.md)               |
| 创建商机         | `lfy-cli pipeline create_pipeline '{...}'`                                                                                                          | [pipeline.md](references/pipeline.md)               |
| 修改商机         | `lfy-cli pipeline update_pipeline '{...}'`                                                                                                          | [pipeline.md](references/pipeline.md)               |
| 添加商机跟进     | `lfy-cli pipeline add_follow_up '{"pipeline_id":123,"content":"<p>...</p>"}'`                                                                       | [pipeline.md](references/pipeline.md)               |
| 销售财年目标     | `lfy-cli report sales_target '{"sales_id":<id>}'`                                                                                                   | [report.md](references/report.md)                   |
| 销售大局观       | `lfy-cli report get_sales_overall '{...}'`                                                                                                          | [report.md](references/report.md)                   |
| GTM 财务报表     | `lfy-cli report get_financial_statements '{"gtm_id":<id>}'`                                                                                         | [report.md](references/report.md)                   |
| 本人信息         | `lfy-cli user get_self '{}'`                                                                                                                        | [user.md](references/user.md)                       |
| 销售名单         | `lfy-cli user get_sales '{}'`                                                                                                                       | [user.md](references/user.md)                       |
| 财年信息         | `lfy-cli ops get_fiscal_year '{}'`                                                                                                                  | [ops.md](references/ops.md)                         |
| 当前周           | `lfy-cli ops get_current_week '{}'`                                                                                                                 | [ops.md](references/ops.md)                         |
| 最近任务         | `lfy-cli schedule get_recent_tasks '{}'`                                                                                                            | [schedule.md](references/schedule.md)               |
| 本周任务         | `lfy-cli schedule get_current_week '{"gtm_id":0,"sales_ids":[],"customer_ids":[],"limit":50}'`                                                      | [schedule.md](references/schedule.md)               |
| 区间任务         | `lfy-cli schedule get_tasks_anytime '{"start_date":"2026-05-01","end_date":"2026-05-18","gtm_id":0,"sales_ids":[],"customer_ids":[],"limit":50}'`   | [schedule.md](references/schedule.md)               |
| 创建任务         | `lfy-cli schedule create_task '{...}'`                                                                                                              | [schedule.md](references/schedule.md)               |
| 联系人列表       | `lfy-cli contact get_list '{...}'`                                                                                                                  | [contact.md](references/contact.md)                 |
| 下拉选项         | `lfy-cli base get_options '{"object_id":<customer_id>,"property":"customer_status\|customer_tags\|customer_region\|customer_industry","cli":true}'` | [base.md](references/base.md)                       |

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
- 只有名称没有 ID 时 → 先 `customer search` / `pipeline search` 拿 `customer_id` / `pipeline_id`，再 `get_details` / `update_*` / `add_follow_up`。
- 记客户/商机跟进 → 无 ID 时先 search；`content` 用 `<p></p>` 包裹。

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


## SKILL 更新
每天使用 SKILL 前进行一次更新检查，按照如下的步骤执行:

1. **查看当前版本version**：读取本文件顶部 frontmatter 中的 version 字段；格式为 MAJOR.MINOR.PATCH
2. **查看最新版本 latest**：`skill_version` 填上一步 frontmatter 的 `version`；`cli_version` 填 `lfy-cli --version` 输出（去掉 `v` 前缀）
```bash
curl -sS -X POST 'https://open-api.6fenyi.com/cli/version' \
  -H 'Content-Type: application/json' \
  -d '{
      "skill_version": "1.0.0",
      "cli_version": "0.18.0"
    }'
```

JSON 返回：

- `skill_latest_version`: 最新版本号（MAJOR.MINOR.PATCH）
- `cli_latest_version`: 最新版本号（MAJOR.MINOR.PATCH）
- `release_note`: 发布说明


3. **更新**：若本地 `skill_version` / `cli_version` 低于 `skill_latest_version` / `cli_latest_version`，分别执行 `npx skills add https://gitee.com/lfy-team/lfy-cli.git -y -g` 与 `npm install -g @6fy/cli@latest && lfy-cli upgrade`，或提示用户更新。必要时，请自动完成升级。