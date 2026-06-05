# 日程任务技能

> `lfy-cli` 是LFY提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli schedule <接口名> '{}'` 与日程系统交互（查询与创建）。

## 注意事项

- **不支持周期任务**：`get_recent_tasks`、`get_current_week` 与 `get_tasks_anytime` 仅面向**非周期性**日程任务；不查询、不展开、不展示重复/周期任务规则下的子任务序列；若用户问「每周例会」「循环任务」等，需说明本命令行能力不包含周期任务
- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息
- `get_recent_tasks` 返回的时间范围为：今天 + 前7天 + 后7天，共15天
- 任务按开始时间排序
- `task_type`、`status_value` 等技术字段默认不展示；`task_id` 仅用于拼任务详情链接，不在列表中单独列出
- 当前版本**不支持修改/删除**已有任务；**支持创建**个人非周期任务，见 [create_task](schedule_create_task.md)
- **任务详情页**：`https://app.6fenyi.com/tasks/{task_id}`（将 `{task_id}` 换为接口返回的 `task_id` 数值）

## 展示格式约定（对话内 Markdown 表格）

日程结果在对话中**使用 Markdown 表格**展示（勿把整段回复包在代码块 ` ``` ` 里）：

- **按日期分组**：每天一个 `####` 标题，其下一张表格（避免单表过长）
- **任务列**：使用可点击链接 `[{task_name}](https://app.6fenyi.com/tasks/{task_id})`（`task_id` 取自 JSON）
- **状态列**：用 emoji 前缀——✅ 已完成（`status_value=30`）· ⚠️ 即将到期或需关注 · ❌ 已过期未完成 · ○ 待开始（`status_value=10`）
- 客户/商机为空时表格对应单元格写「—」；单日超过 15 条只展示前 15 行并注明「当日还有 N 项未列出」
- 单元格内容若含 `|` 字符，需转义或改写，避免破坏表格

---

## 接口列表

### 获取最近日程任务 (get_recent_tasks)

```bash
lfy-cli schedule get_recent_tasks '{}'
```

- 无业务参数，请求体固定为 `{}`（`org_id` / `user_id` 由 lfy-cli-server 从登录态注入）
- 获取最近15天的日程和任务列表（今天 + 前7天 + 后7天），**不含周期任务**（见上方注意事项）

参见 [API 详情](schedule_get_recent_tasks.md)。

### 获取本自然周任务 (get_current_week)

```bash
lfy-cli schedule get_current_week '{"gtm_id":0,"sales_ids":[],"customer_ids":[],"limit": 50}'
```

- 查询本自然周（周一~周日，北京时区）的任务列表，**不含周期任务**（见上方注意事项）
- 支持按 GTM / 销售 / 客户过滤。`sales_ids=[]` 表示查**所有人**（不走权限表）；非空时按 `c.user_id IN (sales_ids) AND user_type=2` 多人过滤；服务端会过滤 `<=0`、去重、截前 50。返回带 `name`、`start_date`、`end_date` 外壳，`tasks[]` 每条含 `date_key`、`status_color`、`tags`、`owners`、关联的客户和商机

参见 [API 详情](schedule_get_current_week.md)。

### 获取指定日期区间任务 (get_tasks_anytime)

```bash
lfy-cli schedule get_tasks_anytime '{"start_date":"2026-05-01","end_date":"2026-05-18","gtm_id":0,"sales_ids":[],"customer_ids":[],"limit":50}'
```

- 查询调用方指定的日期区间（`start_date` ~ `end_date`，含首尾，**最长 60 个自然日**）内的任务列表，**不含周期任务**（见上方注意事项）
- `start_date`、`end_date` 必填；支持按 GTM / 销售 / 客户过滤。`sales_ids=[]` 表示查**所有人**（不走权限表）；非空时按 `c.user_id IN (sales_ids) AND user_type=2` 多人过滤；服务端会过滤 `<=0`、去重、截前 50。返回带 `name`、`start_date`、`end_date` 外壳，`tasks[]` 每条含 `date_key`、`status_color`、`tags`、`owners`、关联的客户和商机

参见 [API 详情](schedule_get_tasks_anytime.md)。

### 创建日程/任务 (create_task)

```bash
lfy-cli schedule create_task '{"task_name":"名称","end_time":"2026-06-07"}'
```

- 创建**非周期**任务；负责人固定为当前登录用户
- `task_name` 必填；`end_time` 必填；`start_time` 可省略（省略时等于 `end_time`）
- `content` 可传但首期不落库

参见 [API 详情](schedule_create_task.md)。

---

## 典型工作流

### 获取最近任务

**经典 query 示例：**
- "最近有什么任务？"
- "帮我看一下这周的日程"
- "查一下最近的日程安排"
- "有哪些任务要完成？"

**流程：**
1. 调用 `get_recent_tasks` 命令获取最近任务
2. 按照截止时间依次展示
3. 通过⚠️提醒用户，截止时间即将到来的任务
4. 通过✅提醒用户，已完成的任务
5. 通过❌提醒用户，已经过期的任务
6. 结合目前记忆中的上下文信息，给出下一步建议


**展示结果：**

按 `due_time` 的日期分组（升序），每天一个四级标题；`{日期标签}` 可用「今天 / 明天 / 昨天」或星期。

📋 最近日程任务（共 {count} 项）

#### {日期标签} {YYYY-MM-DD}

| 状态 | 任务 | 编号 | 类型 | 开始时间 | 截止时间 | 负责人 | 优先级 |
| ---- | ---- | ---- | ---- | -------- | -------- | ------ | ------ |
| {emoji} | [{task_name}](https://app.6fenyi.com/tasks/{task_id}) | #{task_no} | {type_name} | {start_time 或 —} | {due_time} | {owner_name} | {priority_name} |

示例：

#### 今天 2026-06-02

| 状态 | 任务 | 编号 | 类型 | 开始时间 | 截止时间 | 负责人 | 优先级 |
| ---- | ---- | ---- | ---- | -------- | -------- | ------ | ------ |
| ⚠️ | [完成官方文档翻译](https://app.6fenyi.com/tasks/1001) | #001 | 任务 | 2024-03-01 09:00 | 2024-03-05 18:00 | 张三 | 高 |
| ⚠️ | [用户权限优化](https://app.6fenyi.com/tasks/1002) | #002 | 需求 | 2024-03-03 09:00 | 2024-03-10 18:00 | 李四 | 中 |

找不到时（普通段落，勿用代码块）：

目前您没有安排任何任务。您可以访问 LFY 平台创建未来 3 天的工作计划，或让我帮您梳理近期安排。

### 查看本周任务

**经典 query 示例：**
- "本周有哪些任务？"
- "这周我的工作安排"
- "本周销售张三的任务"
- "本周跟 XX 客户相关的任务"

**流程：**
1. 未指定销售时 `sales_ids=[]`（所有人）；指定若干人时先用 `lfy-cli user get_sales` 找到 id 列表，再传给 `sales_ids`
2. 未指定客户时 `customer_ids=[]`；指定时先用 `lfy-cli customer search` 找到 id 列表
3. 调用 `get_current_week`
4. 若 `tasks` 为空，明确告知 "本周暂无任务"
5. 按 `date_key` 分组展示（周一到周日），同一天内按 `due_time` 顺序展示
6. 已完成（`status_value=30`）用 ✅；过期（`due_time < now` 且未完成）用 ❌ 提醒
7. 有 `pipeline_name`/`customer_name` 的任务一并展示关联商机/客户

**展示结果：**

按 `date_key` 从周一到周日分组；同一天内按 `due_time` 升序，每天一张表。

📅 本周任务（{start_date} ~ {end_date}，共 {count} 项）

#### {星期} {date_key}

| 状态 | 任务 | 编号 | 客户 | 商机 | 截止时间 | 负责人 | 优先级 |
| ---- | ---- | ---- | ---- | ---- | -------- | ------ | ------ |
| {emoji} | [{task_name}](https://app.6fenyi.com/tasks/{task_id}) | #{task_no} | {customer_name 或 —} | {pipeline_name 或 —} | {due_time} | {owners 姓名逗号连接} | {priority_name} |

示例：

#### 周三 2026-04-22

| 状态 | 任务 | 编号 | 客户 | 商机 | 截止时间 | 负责人 | 优先级 |
| ---- | ---- | ---- | ---- | ---- | -------- | ------ | ------ |
| ⚠️ | [完成官方文档翻译](https://app.6fenyi.com/tasks/1001) | #1234 | 示例客户 | 示例商机 | 2024-03-05 18:00 | 张三 | 高 |

找不到时（普通段落，勿用代码块）：

本周暂无任务。可以让我帮您规划一下本周的工作重点？

### 指定日期区间任务

**经典 query 示例：**
- "5 月 1 日到 5 月 18 日有哪些任务？"
- "查一下上个月张三的任务安排"
- "这个季度跟 XX 客户相关的任务"

**流程：**
1. 从用户表述中解析 `start_date`、`end_date`（`YYYY-MM-DD`）；若未给出则追问；确认区间不超过 60 个自然日
2. 未指定销售时 `sales_ids=[]`（所有人）；指定若干人时先用 `lfy-cli user get_sales` 找到 id 列表，再传给 `sales_ids`
3. 未指定客户时 `customer_ids=[]`；指定时先用 `lfy-cli customer search` 找到 id 列表
4. 调用 `get_tasks_anytime`
5. 若 `tasks` 为空，明确告知该区间内暂无任务
6. 按 `date_key` 分组展示（从 `start_date` 到 `end_date` 升序），同一天内按 `due_time` 顺序展示
7. 已完成（`status_value=30`）用 ✅；过期（`due_time < now` 且未完成）用 ❌ 提醒
8. 有 `pipeline_name`/`customer_name` 的任务一并展示关联商机/客户

**展示结果：**

按 `date_key` 从 `start_date` 到 `end_date` 分组；同一天内按 `due_time` 升序，每天一张表。

📅 指定区间任务（{start_date} ~ {end_date}，共 {count} 项）

#### {星期或日期标签} {date_key}

| 状态 | 任务 | 编号 | 客户 | 商机 | 截止时间 | 负责人 | 优先级 |
| ---- | ---- | ---- | ---- | ---- | -------- | ------ | ------ |
| {emoji} | [{task_name}](https://app.6fenyi.com/tasks/{task_id}) | #{task_no} | {customer_name 或 —} | {pipeline_name 或 —} | {due_time} | {owners 姓名逗号连接} | {priority_name} |

示例：

#### 2026-05-10

| 状态 | 任务 | 编号 | 客户 | 商机 | 截止时间 | 负责人 | 优先级 |
| ---- | ---- | ---- | ---- | ---- | -------- | ------ | ------ |
| ⚠️ | [完成官方文档翻译](https://app.6fenyi.com/tasks/1001) | #1234 | 示例客户 | 示例商机 | 2024-03-05 18:00 | 张三 | 高 |

找不到时（普通段落，勿用代码块）：

该日期区间内暂无任务。可以换一个区间，或让我帮您查本周/最近任务？

### 创建任务

**经典 query 示例：**
- "帮我创建一个下周三的任务"
- "6 月 10 号提醒我跟进客户"
- "创建一个任务叫 XXX"

**流程：**
1. 从用户表述解析任务名称；若未给出则追问
2. 用户只给**某一天**（无时分）→ `start_time` = `end_time` = 该日（`YYYY-MM-DD`）
3. 用户给**区间** → 分别填 `start_time`、`end_time`
4. 用户提到**几点几分** → 说明 CLI 只支持到自然日，询问是否按该日创建
5. 调用 `create_task`；成功展示任务名、日期、`task_id` 与详情链接
6. 若用户提供了详情 HTML，可说明需在 App 内补充（首期 CLI 不写 content）

**展示结果（普通段落）：**

已为您创建任务「{task_name}」，时间 {start_time} ~ {end_time}。  
详情页：[{task_name}](https://app.6fenyi.com/tasks/{task_id})
