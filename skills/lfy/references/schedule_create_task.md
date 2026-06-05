# create_task（创建日程/任务）

对应 MCP / JSON-RPC：`schedule/create_task`。

## 命令

```bash
lfy-cli schedule create_task '{"task_name":"名称","start_time":"2026-06-04","end_time":"2026-06-07"}'
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
| `content` | string | 可选；**首期不落库**，详情请在 App 内补充 |

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
| 创建成功 | 展示 `task_id`、`create_time`，附链接 `https://app.6fenyi.com/tasks/{task_id}` |

## 成功响应示例

```json
{
  "code": 200,
  "data": {
    "task_id": 111,
    "create_time": "2026-06-04 10:00:05"
  }
}
```

## 常见错误（CLI 为 `Error: ...`）

- 默认状态未配置：`请检查本企业默认事务状态配置是否合理`
- 参数非法：`参数错误`
- 结束早于开始：`结束日期不能早于开始日期`
- 内部错误：`创建失败，请稍后重试`
