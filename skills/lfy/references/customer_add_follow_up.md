# add_follow_up

## 命令

```bash
lfy-cli customer add_follow_up '{"customer_id": 123, "content": "<p>xxx</p>"}'
```

## 参数

| 字段 | 类型 | 说明 |
|------|------|------|
| `customer_id` | int | 必填，>0 |
| `content` | string | 必填，跟进记录内容；**须以 `<p></p>` 包裹**（如 `<p>今日电话沟通，客户意向良好</p>`），trim 后非空，最多 10000 字符 |

## 成功响应示例（MCP 层扁平化，含 code）

```json
{
  "code": 200,
  "follow_up_id": 12344,
  "create_time": "2026-05-08 10:00:00"
}
```

`create_time` 为北京时间 `YYYY-MM-DD HH:mm:ss`。对终端用户仅需确认跟进已记录，**不必展示** `follow_up_id`、`create_time`。

## 常见错误（CLI 为 `Error: ...`）

| 场景 | 文案 |
|------|------|
| 无权限 | `您暂无权限` |
| 客户不存在 | `客户不存在` |
| 参数非法 | `参数错误` / `参数错误：customer_id 错误` / `参数错误：content 不能为空` / `参数错误：content 不能超过 10000 个字符` |
| 其它服务端错误 | `创建失败，请稍后重试` |
