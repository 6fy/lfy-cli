# create_pipeline（创建商机）

对应 MCP / JSON-RPC：`pipeline/create_pipeline`。

## 命令

```bash
lfy-cli pipeline create_pipeline '{"gtm_id":17,"pipeline_name":"示例商机","customer_id":67,"stage_id":78,"sales_id":81,"forecast":9800,"forecast_date":"2026-07-12","tag_ids":[53]}'
```

## 参数

| 字段 | 类型 | 说明 |
|------|------|------|
| `gtm_id` | int | 必填，>0 |
| `pipeline_name` | string | 必填；trim 后非空 |
| `customer_id` | int | 必填，>0 |
| `stage_id` | int | 必填，阶段 ID |
| `sales_id` | int | 可选；≤0 或不传默认当前登录用户对应 ID |
| `forecast` | int | 可选；≤0 时不写入库 |
| `forecast_date` | string | 可选；`YYYY-MM-DD`，空不写库 |
| `tag_ids` | int[] | 可选；写入 `tag_taxonomy_id` |

权限：需商机模块 **create**；且目标客户的 **`sales_id` 须在 create 场景的 `sales_ids` 白名单内**。

## base_api HTTP 响应体

成功：

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "pipeline_id": 179056748370,
    "pipeline_name": "示例商机",
    "created_time": "2026-05-09 10:29:47"
  }
}
```

失败（HTTP 仍为 200，业务码见 `code`）：

```json
{
  "code": 400,
  "message": "您暂无权限",
  "data": null
}
```

经由 `lfy-cli-server` 转发时，仅在 `code==200` 时向 CLI 透出 `data`，故 CLI 侧成功结果示例等价于上面的 `data` 对象。

## 常见错误（CLI 为 `Error: Create: ...`）

- 无权限 / 客户不可见：`您暂无权限`
- 参数非法：`参数错误`
- 其它：`创建失败，请稍后重试`
