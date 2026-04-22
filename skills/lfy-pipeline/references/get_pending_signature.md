# get_pending_signature — 获取最近待签单商机

## 命令

```bash
lfy-cli pipeline get_pending_signature '{"stage": 0, "page_size": 10, "page": 1}'
```

## 参数

| 参数名      | 类型    | 必填 | 默认 | 说明 |
| ----------- | ------- | ---- | ---- | ---- |
| `stage`     | integer | 否   | 0    | 0=全部阶段；>0 对应 `b_config_pl_phase.logic_phase`（10/20/…/100） |
| `page_size` | integer | 否   | 10   | 每页数量，<=0 取 10，>100 截断为 100 |
| `page`      | integer | 否   | 1    | 页码，从 1 开始，<1 当 1 |

## 成功响应

```json
{
  "name": "最近待签单商机",
  "total": 21,
  "pipelines": [
    {
      "pipeline_id": 290349102942,
      "pipeline_name": "新项目",
      "forecast_amount": 800000.00,
      "forecast_set": true,
      "customer_name": "深圳创新科技",
      "stage_value": 80,
      "stage_name": "方案报价",
      "stage_checklist": {
        "completed_count": 1,
        "total_count": 5,
        "completion_rate": 0.20
      },
      "ai_suggestion": ""
    }
  ]
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | string | 固定 `"最近待签单商机"` |
| `total` | integer | 全量匹配记录数（与分页无关） |
| `pipelines` | array | 当前页数据；无则为 `[]` |
| `pipelines[].pipeline_id` | integer | 商机 ID |
| `pipelines[].pipeline_name` | string | 商机名称 |
| `pipelines[].forecast_amount` | number | 预测金额；NULL → 0 |
| `pipelines[].forecast_set` | boolean | 是否已填预测金额 |
| `pipelines[].customer_name` | string | 客户名称 |
| `pipelines[].stage_value` | integer | 阶段百分比 |
| `pipelines[].stage_name` | string | 阶段名 |
| `pipelines[].stage_checklist.completed_count` | integer | 当前阶段已完成任务数 |
| `pipelines[].stage_checklist.total_count` | integer | 当前阶段任务总数 |
| `pipelines[].stage_checklist.completion_rate` | number | 完成率；total=0 时 0，否则保留两位小数 |
| `pipelines[].ai_suggestion` | string | AI 建议；本期固定空串 |

## 权限

以请求 `user_id` 作为销售 ID 直接过滤：`p.sales_id = user_id`。不走 per_user 权限表。若 user_id 不是销售，自然返回 `total:0, pipelines:[]`。

## 错误处理

| 文案 | 含义 |
| ---- | ---- |
| `您暂无权限` | 参数非法 / 服务端异常的统一兜底 |
