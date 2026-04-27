---
name: lfy-schedule
description: 日程任务查询技能。适用于获取最近两周或本自然周的日程和任务信息。当用户需要查看近期或本周任务安排时使用此技能。
version: 1.3.0
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli schedule --help"
---

# 日程任务查询技能

> `lfy-cli` 是陆份仪提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli schedule <接口名> '{}'` 与日程系统交互。

## 注意事项

- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息
- 返回的时间范围为：今天 + 前7天 + 后7天，共15天
- 任务按开始时间排序
- `task_id`、`task_type`、`status_value` 等技术字段默认不展示
- 当前版本不支持对日程任务进行任何修改操作
- 访问日程任务页面：https://app.6fenyi.com/tasks/{{task_id}}

---

## 接口列表

### 获取最近日程任务 (get_recent_tasks)

```bash
lfy-cli schedule get_recent_tasks '{}'
```

获取最近15天的日程和任务列表（今天 + 前7天 + 后7天）。

参见 [API 详情](references/get_recent_tasks.md)。

### 获取本自然周任务 (get_current_week)

```bash
lfy-cli schedule get_current_week '{"gtm_id":0,"sales_ids":[],"customer_ids":[],"limit":50}'
```

查询本自然周（周一~周日，北京时区）的任务列表，支持按 GTM / 销售 / 客户过滤。`sales_ids=[]` 表示查**所有人**（不走权限表）；非空时按 `c.user_id IN (sales_ids) AND user_type=2` 多人过滤；服务端会过滤 `<=0`、去重、截前 50。返回带 `name`、`start_date`、`end_date` 外壳，`tasks[]` 每条含 `date_key`、`status_color`、`tags`、`owners`、关联的客户和商机。

参见 [API 详情](references/get_current_week.md)。

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

📋 最近日程任务（共<count>项）：

| 编号 | 任务名称 | 类型 | 开始时间 | 截止时间 | 负责人 | 状态 | 优先级 |
|------|---------|------|---------|---------|-------|------|--------|
| #001 | 完成官方文档翻译 | 任务 | 2024-03-01 09:00 | 2024-03-05 18:00 | 张三 | 进行中 | 高 |
| #002 | 用户权限优化 | 需求 | 2024-03-03 09:00 | 2024-03-10 18:00 | 李四 | 进行中 | 中 |

找不到时：

```
目前您没有安排任何任务，您可以访问陆份仪平台创建自己未来3天的工作计划，或者让我来帮你梳理一下未来3天的工作计划？
```

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

**展示建议：**

📅 本周任务（`<start_date>` ~ `<end_date>`，共 `<count>` 项）：

**周一 `<YYYY-MM-DD>`**

| 编号 | 任务 | 客户 | 商机 | 截止 | 负责人 | 状态 | 优先级 |
|------|------|------|------|------|--------|------|--------|
| #`<task_no>` | `<task_name>` | `<customer_name>` | `<pipeline_name>` | `<due_time>` | `<owner.name列表>` | `<status_name>` | `<priority_name>` |

找不到时：

```
本周暂无任务。可以让我帮你规划一下本周的工作重点？
```
