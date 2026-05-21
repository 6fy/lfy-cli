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
| `updates` | object | 必填；**仅出现的键会更新**；未出现的键视为不修改；未知键报错；**空对象 `{}`**：不写库、不刷新 ES，成功返回当前 `updated_time` |

### `updates` 完整示例

```json
{
  "projectname": "修改商机",
  "forecast": 100,
  "forecast_date": "2026-12-12",
  "forecast_start_date": "2026-12-12",
  "sales_id": 1,
  "status_id": 23123,
  "tags": "123,456",
  "win_possibility": 287705087406202,
  "actual": 100,
  "stage_id": 123
}
```

### `updates` 支持的键（均可选组合）

| 键 | 类型 | 规则 |
|----|------|------|
| `projectname` | string | 商机名称；trim 后非空，≤50 字符（Unicode） |
| `forecast` | number | 预测金额；≥0，最多两位小数 |
| `forecast_date` | string | 预测签单时间；`YYYY-MM-DD`；**空字符串 `""` 表示清空** |
| `forecast_start_date` | string | 启动时间；`YYYY-MM-DD`；**空字符串 `""` 表示清空** |
| `sales_id` | int | 负责人 id；>0；选项见 `lfy-cli user get_sales`（`'{}'`）；**≤0 或非法报参数错误** |
| `status_id` | int | 商机状态；>0；选项见 `lfy-cli base get_options`（`{"object_id": <pipeline_id>, "property": "pipeline_status"}`）；**≤0 或非法报参数错误** |
| `tags` | string | 逗号分隔标签 id；**空字符串 `""` 表示清空**；选项见 `lfy-cli base get_options`（`{"object_id": <pipeline_id>, "property": "pipeline_tags"}`） |
| `win_possibility` | int | 签单可能性；>0；选项见 `lfy-cli base get_options`（`{"object_id": <pipeline_id>, "property": "pipeline_win_possibility"}`）；**≤0 或非法报参数错误** |
| `actual` | number | 实际签单金额；合法数字即可，最多两位小数（可为 0 或负数） |
| `stage_id` | int | 阶段 id；>0，传 `get_sales_stage` 返回的 `stage_id`；选项见 `lfy-cli pipeline get_sales_stage`（`{"gtm_id": <gtm_id>}`）；**≤0 或非法报参数错误** |

> **说明**：`object_id` 均填当前商机的 `pipeline_id`。ID 类字段（`stage_id`、`sales_id`、`status_id`、`win_possibility`）JSON 反序列化失败或 ≤0 时报参数错误。`delivery_status`、`revenue_status`、`phase` 等未列出的键视为未知字段，报参数错误。

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
| JSON/字段非法 | `参数错误` |
| 其它服务端错误 | `更新失败，请稍后重试` |
