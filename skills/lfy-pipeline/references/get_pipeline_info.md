# get_pipeline_info — 获取商机详情

## 命令

```bash
lfy-cli pipeline get_pipeline_info '{"pipeline_id": <pipeline_id>}'
```

## 参数

| 参数名         | 类型    | 必填 | 说明     |
| -------------- | ------- | ---- | -------- |
| `pipeline_id` | integer | 是   | 商机 ID  |

## 成功响应（字段说明）

响应为单一 JSON 对象（与接口契约一致），主要字段：

| 字段名                     | 类型 | 说明 |
| -------------------------- | ------- | ---- |
| `pipeline_id`              | integer | 商机 ID |
| `pipeline_name`            | string  | 商机名称 |
| `forecasted_deal_amount`   | number  | 预测签单金额 |
| `forecasted_close_date`    | string  | 预测签单日期 `YYYY-MM-DD`，可空则为 `""` |
| `sales_owner`              | string  | 销售负责人 |
| `pipeline_status`          | string  | 商机状态展示名 |
| `customer_id`              | integer | 客户 ID |
| `customer_name`            | string  | 客户名称 |
| `create_time`              | string  | 创建日期 `YYYY-MM-DD`，可空则为 `""` |
| `recommended_cycle_days`   | integer | 推荐签单周期天数 |
| `current_sales_stage`      | object 或 null | 当前销售阶段；无匹配阶段时为 `null` |
| `pipeline_contacts`        | array   | 商机侧联系人（含 `role`） |
| `customer_contacts`        | array   | 客户侧联系人（无 `role`） |

`current_sales_stage` 对象字段：`due_date`、`stage_name`、`milestone_goal`、`value_proposition`、`suggested_stage_days`、`elapsed_days`（日期为 `YYYY-MM-DD` 字符串）。

## 错误处理接口通过 `error_message` 区分场景（CLI 以 `Error: …` 展示）：

| 文案 | 含义 |
| ---- | ---- |
| `您没有商机模块的权限` | 无商机模块 detail 权限 |
| `商机不存在` | 商机不存在或未激活等 |
| `您没有访问此商机的权限` | 有模块权限但负责人不在可见范围 |
| `您暂无权限` | 参数非法或其它服务端错误（统一兜底） |
