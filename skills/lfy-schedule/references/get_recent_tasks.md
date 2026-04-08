# get_recent_tasks — 获取最近两周的日程任务

## 接口说明

获取最近15天的日程和任务列表，包括今天、前7天和后7天。返回结果按开始时间排序。

## 请求示例

```bash
lfy-cli schedule get_recent_tasks '{}'
```

## 返回示例

```json
[
  {
    "task_id": 1001,
    "task_no": "TASK-2024-001",
    "task_type": 1,
    "type_name": "任务",
    "task_name": "完成官方文档翻译",
    "start_time": "2024-03-01 09:00",
    "due_time": "2024-03-05 18:00",
    "owner_name": "张三",
    "status_value": 20,
    "status_name": "进行中",
    "priority_name": "高"
  }
]
```

## 字段说明

| 字段           | 类型    | 说明                      |
| -------------- | ------- | ------------------------- |
| `task_id`      | integer | 任务唯一 ID               |
| `task_no`      | string  | 任务编号                  |
| `task_type`    | integer | 任务类型（见下方说明）    |
| `type_name`    | string  | 类型名称                  |
| `task_name`    | string  | 任务名称                  |
| `start_time`   | string  | 开始时间，格式：YYYY-MM-DD HH:mm |
| `due_time`     | string  | 截止时间，格式：YYYY-MM-DD HH:mm |
| `owner_name`   | string  | 负责人姓名                |
| `status_value` | integer | 状态值（见下方说明）      |
| `status_name`  | string  | 状态名称                  |
| `priority_name` | string  | 优先级名称                |

## 任务类型 (task_type)

| 值   | 类型名称 |
| ---- | -------- |
| 1    | 任务     |
| 2    | 需求     |
| 3    | 缺陷     |

## 状态值 (status_value)

| 值   | 状态名称 |
| ---- | -------- |
| 10   | 待办     |
| 20   | 进行中   |
| 30   | 已完成   |
