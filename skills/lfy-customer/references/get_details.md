# get_details — 获取客户详情

## 命令

```bash
lfy-cli customer get_details '{"customer_id": <customer_id>}'
```

## 参数

| 参数名         | 类型    | 必填 | 说明     |
| -------------- | ------- | ---- | -------- |
| `customer_id` | integer | 是   | 客户 ID  |

## 成功响应（字段说明）

响应为单一 JSON 对象，主要字段：

| 字段名                       | 类型    | 说明 |
| ---------------------------- | ------- | ---- |
| `customer_id`                | integer | 客户 ID |
| `customer_name`              | string  | 客户名称 |
| `sales_id`                   | integer | 销售负责人 ID |
| `sales_owner`                | string  | 销售负责人姓名 |
| `customer_alias`             | string  | 客户别名 |
| `customer_status`            | string  | 客户状态 |
| `annual_procurement_amount`  | number  | 年采购金额 |
| `customer_no`                | string  | 客户编号/自编码 |
| `maturity`                   | string  | 成熟度，如 `"0%(兴趣寥寥)"` |
| `tags`                       | array   | 客户标签列表（字符串数组） |
| `gtm_id`                     | integer | GTM ID |
| `gtm_name`                   | string  | GTM 名称 |
| `domain`                     | string  | 域名 |
| `industry`                   | string  | 行业 |
| `region`                     | string  | 区域 |
| `create_time`                | string  | 创建日期 `YYYY-MM-DD` |
| `pipelines`                  | object  | 商机列表包装 |
| `contacts`                   | array   | 联系人列表 |
| `previous_followup_records`  | array   | 跟进记录列表 |
| `schedule`                   | array   | 客户相关近期任务列表（过期30天~未来30天，最多50条） |

### pipelines 对象

| 字段名  | 类型    | 说明 |
| ------- | ------- | ---- |
| `total` | integer | 商机总数 |
| `items` | array   | 商机列表 |

每条商机包含：`pipeline_id`、`pipeline_name`、`sales_stage`、`pipeline_status`、`actual_deal_amount`、`forecasted_deal_amount`、`create_time`。

### contacts 数组元素

| 字段名     | 类型   | 说明 |
| ---------- | ------ | ---- |
| `name`     | string | 姓名 |
| `position` | string | 职位 |
| `mobile`   | string | 手机号（多值逗号分隔） |
| `email`    | string | 邮箱（多值逗号分隔） |

### previous_followup_records 数组元素

| 字段名       | 类型   | 说明 |
| ------------ | ------ | ---- |
| `sales_name` | string | 销售名称 |
| `content`    | string | 事件内容 |
| `create_time`| string | 创建时间 `YYYY-MM-DD HH:mm` |

### schedule 数组元素

| 字段名       | 类型    | 说明 |
| ------------ | ------- | ---- |
| `task_id`    | integer | 任务 ID |
| `task_no`    | string  | 任务编号 |
| `task_name`  | string  | 任务名称 |
| `task_status`| string  | 状态名（如 "待办"、"进行中"、"已完成"） |
| `start_time` | string  | 开始时间 `YYYY-MM-DD HH:mm`；无值时为 `""` |
| `due_time`   | string  | 截止时间 `YYYY-MM-DD HH:mm`；无值时为 `""` |
| `owner_name` | string  | 负责人姓名 |

## 错误处理

接口通过 `error_message` 区分场景（CLI 以 `Error: …` 展示）：

| 文案 | 含义 |
| ---- | ---- |
| `您没有客户模块的权限` | 无客户模块 detail 权限 |
| `客户不存在` | 客户不存在或未激活 |
| `您没有访问此客户的权限` | 有模块权限但负责人不在可见范围 |
| `您暂无权限` | 参数非法或其它服务端错误（统一兜底） |
