# get_current_week — 获取本自然周任务

## 接口说明

查询本自然周（周一~周日，北京时区）的任务列表。主查询一次 SQL 拿任务主行，再按 `task_id` 批量查负责人，最终装配为带 `date_key` / `owners` 的结构。排序 `pd_date ASC, task_id ASC`。

## 命令

```bash
lfy-cli schedule get_current_week '{"gtm_id":0,"sales_id":0,"customer_ids":[],"limit":50}'
```

## 参数

| 参数名         | 类型      | 必填 | 默认 | 说明 |
| -------------- | --------- | ---- | ---- | ---- |
| `gtm_id`       | integer   | 否   | 0    | 0=全部；>0 按 GTM 过滤（需任务挂在客户下） |
| `sales_id`     | integer   | 否   | 0    | 0=查所有人（不走权限表）；>0 按负责人精确过滤 |
| `customer_ids` | integer[] | 否   | []   | 空数组=不过滤；非空按客户 ID 精确过滤 |
| `limit`        | integer   | 否   | 50   | <=0 取 50，>100 截为 100 |

## 成功响应

```json
{
  "name": "本自然周任务",
  "start_date": "2026-04-20",
  "end_date": "2026-04-26",
  "tasks": [
    {
      "date_key": "2026-04-22",
      "task_id": 1001,
      "task_no": "1234",
      "task_type": 1,
      "type_name": "任务",
      "task_name": "完成官方文档翻译",
      "start_time": "2024-03-01 09:00:00",
      "due_time": "2024-03-05 18:00:00",
      "status_value": 20,
      "status_name": "进行中",
      "priority_name": "高",
      "customer_id": 0,
      "customer_name": "",
      "pipeline_id": 0,
      "pipeline_name": "",
      "owners": [{"id": 1, "name": "张三"}]
    }
  ]
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | string | 固定 `"本自然周任务"` |
| `start_date` | string | 本周一 `YYYY-MM-DD` |
| `end_date` | string | 本周日 `YYYY-MM-DD`（= `start_date + 6 days`） |
| `tasks` | array | 任务数组；无数据为 `[]` |
| `tasks[].date_key` | string | 从 `due_time` 提取的 `YYYY-MM-DD` |
| `tasks[].task_id` | integer | 任务 ID |
| `tasks[].task_no` | string | 任务编号 |
| `tasks[].task_type` | integer | `1` 任务 / `2` 需求 / `3` 缺陷 |
| `tasks[].type_name` | string | 类型中文名 |
| `tasks[].task_name` | string | 任务名称 |
| `tasks[].start_time` | string | `YYYY-MM-DD HH:mm:ss`，可能为 `""` |
| `tasks[].due_time` | string | `YYYY-MM-DD HH:mm:ss`，不会为 `""` |
| `tasks[].status_value` | integer | `10` 待开始 / `20` 进行中 / `30` 已完成 |
| `tasks[].status_name` | string | 状态中文名 |
| `tasks[].priority_name` | string | 优先级中文名 |
| `tasks[].customer_id` | integer | 客户 ID；未挂 0 |
| `tasks[].customer_name` | string | 客户名；未挂 `""` |
| `tasks[].pipeline_id` | integer | 商机 ID；未挂 0 |
| `tasks[].pipeline_name` | string | 商机名；未挂 `""` |
| `tasks[].owners` | array | 负责人列表；无则 `[]` |
| `tasks[].owners[].id` | integer | 用户 ID |
| `tasks[].owners[].name` | string | 用户名 |

## 任务类型 (task_type)

| 值 | 类型 |
|----|------|
| 1  | 任务 |
| 2  | 需求 |
| 3  | 缺陷 |

## 状态值 (status_value)

| 值 | 名称   |
|----|--------|
| 10 | 待开始 |
| 20 | 进行中 |
| 30 | 已完成 |

## 权限

`sales_id=0` 时**完全不做负责人过滤**（返回全公司本周任务）；`sales_id=X(X>0)` 时按 `bp_task_collaborator.user_id = X AND user_type = 2` 精确匹配。不走 `per_user` 权限表。

> `gtm_id>0` 或 `customer_ids` 非空时，由于 WHERE 引用 `cust_assoc`，**没挂客户的任务会被排除**。这是预期行为（"看某 gtm/某些客户下的任务"）。

## 错误处理

| 文案 | 含义 |
| ---- | ---- |
| `您暂无权限` | 参数非法 / 服务端异常的统一兜底 |
