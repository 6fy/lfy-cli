# get_recent_tasks — 获取最近日程任务

## 接口说明

按当前登录用户权限，返回**最近 15 天**（今天 + 前 7 天 + 后 7 天，北京时区）内的日程与任务列表。底层为 `TaskService.GetRecentTasks`，与 `per_user` 等既有权限逻辑一致。

**不支持周期任务**：本接口仅返回非周期任务数据，不包含重复/周期规则下的任务展开或专项查询。

## 命令

```bash
lfy-cli schedule get_recent_tasks '{}'
```

## 参数

| 说明 | 值 |
| ---- | -- |
| 请求体 | 固定为 `{}`（无业务字段；`org_id`、`user_id` 由 lfy-cli-server 从登录态传至 base_api） |

## 成功响应

返回 `model.TaskInfo` 数组（字段以 swagger / 服务实现为准），按业务约定排序。

## 错误处理

| 文案 | 含义 |
| ---- | ---- |
| `Error: ...` | lfy-cli 标准错误格式 |
| `您暂无权限` | 无权限等统一兜底（若接口层返回） |
