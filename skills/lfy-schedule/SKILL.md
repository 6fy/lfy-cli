---
name: lfy-schedule
description: 日程任务查询技能。适用于获取最近两周的日程和任务信息。当用户需要查看近期任务安排时使用此技能。
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

---

## 接口列表

### get_recent_tasks — 获取最近两周的日程任务

```bash
lfy-cli schedule get_recent_tasks '{}'
```

获取最近15天的日程和任务列表（今天 + 前7天 + 后7天）。

参见 [API 详情](references/get_recent_tasks.md)。

---

## 返回格式

### get_recent_tasks 返回格式

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

| 字段           | 类型    | 说明                      |
| -------------- | ------- | ------------------------- |
| `task_id`      | integer | 任务唯一 ID               |
| `task_no`      | string  | 任务编号                  |
| `task_type`    | integer | 任务类型                  |
| `type_name`    | string  | 类型名称                  |
| `task_name`    | string  | 任务名称                  |
| `start_time`   | string  | 开始时间                  |
| `due_time`     | string  | 截止时间                  |
| `owner_name`   | string  | 负责人姓名                |
| `status_value` | integer | 状态值                    |
| `status_name`  | string  | 状态名称                  |
| `priority_name` | string  | 优先级名称               |

### task_type 任务类型说明

| 值   | 类型名称 |
| ---- | -------- |
| 1    | 任务     |
| 2    | 需求     |
| 3    | 缺陷     |

### status_value 状态值说明

| 值   | 状态名称 |
| ---- | -------- |
| 10   | 待办     |
| 20   | 进行中   |
| 30   | 已完成   |

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
2. 按任务类型或时间分组展示
3. 展示任务详情包括状态、优先级

**展示结果：**

有任务时：

📋 最近日程任务（共<count>项）：

**进行中**

| 编号 | 任务名称 | 类型 | 开始时间 | 截止时间 | 负责人 | 状态 | 优先级 |
|------|---------|------|---------|---------|-------|------|--------|
| TASK-2024-001 | 完成官方文档翻译 | 任务 | 2024-03-01 09:00 | 2024-03-05 18:00 | 张三 | 进行中 | 高 |
| TASK-2024-002 | 用户权限优化 | 需求 | 2024-03-03 09:00 | 2024-03-10 18:00 | 李四 | 进行中 | 中 |

**字段映射**：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| task_no | 编号 | 默认展示 |
| task_name | 任务名称 | 默认展示 |
| type_name | 类型 | 默认展示 |
| start_time | 开始时间 | 默认展示 |
| due_time | 截止时间 | 默认展示 |
| owner_name | 负责人 | 默认展示 |
| status_name | 状态 | 默认展示 |
| priority_name | 优先级 | 默认展示 |
| task_id | - | 技术字段，默认隐藏 |
| task_type | - | 技术字段，默认隐藏 |
| status_value | - | 技术字段，默认隐藏 |

未找到时：

未找到最近15天内的日程任务。
