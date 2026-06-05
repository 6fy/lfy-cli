# create_task（创建日程/任务）

对应 MCP / JSON-RPC：`schedule/create_task`。

## 命令

```bash
lfy-cli schedule create_task '{"task_name":"名称","start_time":"2026-06-04","end_time":"2026-06-07"}'
```

带任务详情：

```bash
lfy-cli schedule create_task '{"task_name":"名称","end_time":"2026-06-07","content":"<p>任务详细内容</p>"}'
```

仅给截止日期时（口语「下周三的任务」）：

```bash
lfy-cli schedule create_task '{"task_name":"名称","end_time":"2026-06-12"}'
```

## 参数

| 字段 | 类型 | 说明 |
|------|------|------|
| `task_name` | string | 必填；trim 后 1～30 字符（Unicode） |
| `start_time` | string | 可选；`YYYY-MM-DD`；省略时等于 `end_time` |
| `end_time` | string | 必填；`YYYY-MM-DD` |
| `content` | string | 可选；HTML 详情，≤ 10000 Unicode 字符；非空时写入 MongoDB 并关联 `detail_id` |

## 限制

- **不支持**周期/重复任务
- **仅能为当前登录用户**创建（不能指定他人为负责人）
- 只支持到**自然日**，不支持几点几分

## Agent 工作流

| 用户表达 | Agent 行为 |
|----------|------------|
| 只给某一天（无时分） | `start_time` = `end_time` = 该日 |
| 给起止日期区间 | 分别填入 `start_time`、`end_time` |
| 提到几点几分 | 说明 CLI 只支持到自然日，询问是否按该日创建 |
| 用户提供任务详情 | 填入 `content`（HTML 允许） |
| 创建成功 | 展示 `task_id`、`create_time`、`detail_saved`，附链接 `https://app.6fenyi.com/tasks/{task_id}` |
| `detail_saved=false` | 告知任务已创建但详情未保存，转述 `warning_message` |

## 成功响应示例

完全成功：

```json
{
  "code": 200,
  "data": {
    "task_id": 111,
    "create_time": "2026-06-04 10:00:05",
    "detail_saved": true
  }
}
```

部分成功（有 `content` 但 Mongo 写入失败，任务仍创建）：

```json
{
  "code": 200,
  "data": {
    "task_id": 111,
    "create_time": "2026-06-04 10:00:05",
    "detail_saved": false,
    "warning_message": "任务详情保存失败，任务已创建"
  }
}
```

## 常见错误（CLI 为 `Error: ...`）

- 默认状态未配置：`请检查本企业默认事务状态配置是否合理`
- 参数非法（含 content 超长）：`参数错误`
- 结束早于开始：`结束日期不能早于开始日期`
- 内部错误：`创建失败，请稍后重试`
