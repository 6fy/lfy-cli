# get_sales_overall — 销售大局观（当前财年）

## 接口说明

查询**企业当前财年**内三条按日时间序列：

- `sum_actual`：实际签单金额（仅返回 `amount > 0` 的日期）。
- `sum_forecast`：预测签单金额（仅返回 `amount > 0` 的日期）。
- `total_opportunity`：商机池快照（Elasticsearch `b_pl_analysis_log` 按日聚合后，**仅当相对上一输出日的 `count` 或 `total_amount` 发生变化时才输出该日**，连日数值相同则合并省略，减少无效重复点）。

财年起止由服务端根据 `b_o_money` 当前财年记录自动解析，调用方无需传财年。

**权限**：需具备商机（`category_id=2`）**list** 或 **detail** 任一有效权限；否则返回 `error_message: 您暂无权限`。

## 请求示例

```bash
lfy-cli report get_sales_overall '{"gtm_id": 0, "sales_id": 0, "customer_ids": []}'
lfy-cli report get_sales_overall '{"gtm_id": 1001, "sales_id": 0, "customer_ids": [1111, 2222]}'
```

### 请求体字段

| 字段 | 类型 | 必填 | 说明 |
| --- | --- | --- | --- |
| `gtm_id` | number | 否 | `0` 或省略 = 不按 GTM 过滤 |
| `sales_id` | number | 否 | `0` 或省略 = 不按销售过滤 |
| `customer_ids` | number[] | 否 | 空数组或省略 = 不按客户过滤 |

## 返回示例（成功）

```json
{
  "sum_actual": [
    {"date": "2026-01-15", "amount": 125000.0}
  ],
  "sum_forecast": [
    {"date": "2026-06-15", "amount": 98000.0}
  ],
  "total_opportunity": [
    {"date": "2026-01-01", "total_amount": 3610270.0, "count": 51}
  ]
}
```

## 字段说明

### sum_actual / sum_forecast

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `date` | string | `YYYY-MM-DD` |
| `amount` | number | 当日汇总金额 |

### total_opportunity

| 字段 | 类型 | 说明 |
| --- | --- | --- |
| `date` | string | `YYYY-MM-DD` |
| `total_amount` | number | 当日 `Forecast` 求和 |
| `count` | number | 当日命中文档数（`doc_count`） |

**稀疏规则**：按日期升序遍历；第一条有效数据必输出；之后仅当 `(count, total_amount)` 与**上一条已输出**的取值不同时再输出（表示池子相对前一日发生了变化）。

## 错误示例

```json
{"error_message": "您暂无权限"}
```

```json
{"error_message": "暂未配置当前财年"}
```

```json
{"error_message": "查询失败"}
```
