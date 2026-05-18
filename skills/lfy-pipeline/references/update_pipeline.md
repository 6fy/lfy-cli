# update_pipeline

## 命令

```bash
lfy-cli pipeline update_pipeline '{"pipeline_id": 111, "updates": {}}'
```

## 权限

需要 per_user 商机模块 **detail** 场景有效权限，且目标商机的 `sales_id` 须在 detail 的 `sales_ids` 白名单内（与「查看商机详情」门禁一致）。

## 参数

| 字段 | 类型 | 说明 |
|------|------|------|
| `pipeline_id` | int | 必填，>0 |
| `updates` | object | 必填；**仅出现的键会更新**；未知键报错；**空对象 `{}`**：不写库、不刷新 ES，成功返回当前 `updated_time` |

### `updates` 支持的键（均可选组合）

| 键 | 类型 | 规则 |
|----|------|------|
| `projectname` | string | trim 后非空，≤50 字符（Unicode） |
| `forecast` | number | ≥0，最多两位小数 |
| `forecast_date` | string | `YYYY-MM-DD`；**空字符串 `""` 表示清空** |
| `forecast_start_date` | string | `YYYY-MM-DD`；空字符串清空 |
| `phase` | int | >0；选项见 `lfy-cli pipeline get_sales_stage`。**≤0 或非整数自动跳过该项** |
| `win_possibility` | int | >0；选项见 `lfy-cli base get_options`（`property=pipeline_win_possibility`）。**非法跳过** |
| `sales_id` | int | >0。**非法跳过** |
| `delivery_status` | int | >0；选项见 `property=pipeline_delivery_status`。**非法跳过** |
| `revenue_status` | int | >0；选项见 `property=pipeline_revenue_status`。**非法跳过** |
| `tags` | string | 逗号分隔标签 **taxonomy id**；**空字符串清空**；选项见 `property=pipeline_tags` |

## base_api HTTP 响应体

成功：

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "pipeline_id": 181220445914,
    "updated_time": "2026-05-18 22:20:54"
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

`updated_time` 为北京时间 `YYYY-MM-DD HH:mm:ss`。

## 常见错误（CLI 为 `Error: UpdatePipeline: ...`）

| 场景 | message |
|------|------|
| 模块无权限或销售范围无权限 | `您暂无权限` |
| 商机不存在或未激活 | `商机不存在` |
| JSON/字段非法（projectname/forecast/date/tags/未知键） | `参数错误` |
| 其它服务端错误 | `更新失败，请稍后重试` |
