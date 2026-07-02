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
| `start_time` | string | 可选；`YYYY-MM-DD` 或 `YYYY-MM-DD HH:mm`；省略时等于 `end_time`；须与 `end_time` 同粒度 |
| `end_time` | string | 必填；`YYYY-MM-DD` 或 `YYYY-MM-DD HH:mm` |
| `content` | string | 可选；HTML 详情，≤ 10000 Unicode 字符；非空时写入 MongoDB 并关联 `detail_id` |

## 时间格式约束（重要，别理解错）

- 只接受两种格式：`YYYY-MM-DD`（纯日期）或 `YYYY-MM-DD HH:mm`（到分钟）。
- `start_time` 与 `end_time` **必须同粒度**：要么都带 `HH:mm`，要么都不带；一个带一个不带 → 报错。
- **不支持秒**：`YYYY-MM-DD HH:mm:ss` 会返回 `参数错误`。
- 用户明确提到几点几分 → 用 `YYYY-MM-DD HH:mm`；否则用 `YYYY-MM-DD`。

### 纯日期（`YYYY-MM-DD`）
- `start_time` 可省略（省略时等于 `end_time`）；允许起止同一天。

### 带时分（`YYYY-MM-DD HH:mm`）——额外硬性要求
- **`start_time` 必填且必须也带 `HH:mm`**：只要 `end_time` 到分钟，就必须同时传带 `HH:mm` 的 `start_time`，**不能省略**、不能只传日期。
- **不能是同一时刻**，且 **`end_time` 必须比 `start_time` 至少晚 15 分钟**（正好 15 分钟可以）。
- 若用户只说了一个时间点（如「下午 3 点提醒我」）而没有明确结束时间：应向用户确认时长，或按合理默认给出比开始至少晚 15 分钟的 `end_time`（如默认 30 分钟），不要把 `start_time`、`end_time` 填成同一时刻。
- 带时分示例：`lfy-cli schedule create_task '{"task_name":"评审会","start_time":"2026-06-07 14:00","end_time":"2026-06-07 15:30"}'`

## 限制

- **不支持**周期/重复任务
- **仅能为当前登录用户**创建（不能指定他人为负责人）
- 时间精度到**分钟**，不支持秒

## Agent 工作流

| 用户表达 | Agent 行为 |
|----------|------------|
| 只给某一天（无时分） | `start_time` = `end_time` = 该日（`YYYY-MM-DD`） |
| 给起止日期区间 | 分别填入 `start_time`、`end_time`（两端同粒度） |
| 提到几点几分 | 用 `YYYY-MM-DD HH:mm`；**必须同时给带时分的 `start_time` 和 `end_time`**，且 `end_time` 比 `start_time` 至少晚 15 分钟（不能相同） |
| 只说一个时间点、没说时长 | 先确认时长，或按默认（如 30 分钟）设 `end_time`；禁止把起止填成同一时刻 |
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
- 参数非法（含 content 超长、时间带秒）：`参数错误`
- 两端时间粒度不一致：`start_time 与 end_time 时间格式必须一致（同为 YYYY-MM-DD 或同为 YYYY-MM-DD HH:mm）`
- 带时分却未传 `start_time`：`带时分创建任务时必须提供 start_time（格式 YYYY-MM-DD HH:mm）`
- 带时分且结束不比开始晚至少 15 分钟（含相同）：`结束时间需比开始时间至少晚15分钟`
- 结束早于开始：`结束日期不能早于开始日期`
- 内部错误：`创建失败，请稍后重试`
