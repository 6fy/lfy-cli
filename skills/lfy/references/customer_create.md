# create_customer

## 命令

```bash
lfy-cli customer create_customer '{"gtm_id": 1, "customer_name": "示例客户", "sales_id": 0}'
```

## 参数

| 字段 | 类型 | 说明 |
|------|------|------|
| `gtm_id` | int | 必填，>0，可先 `get_gtms` |
| `customer_name` | string | 必填，自动 trim，1～50 字符（Unicode），企业内激活客户不可重名 |
| `sales_id` | int | 可选；0 或不传则默认为自己（当前登录 `user_id`）；须在 per_user **create** 场景 `sales_ids` 白名单内 |

## 成功响应示例

```json
{
  "customer_id": 111,
  "customer_name": "示例客户",
  "created_time": "2026-05-08 10:00:00"
}
```

## 常见错误（CLI 为 `Error: ...`）

- 无 create 权限或负责人不在白名单：`您暂无权限`
- 名称重复：`客户名称已存在`
- GTM 无效：`GTM 不存在或未启用`
- 参数非法：`参数错误`
