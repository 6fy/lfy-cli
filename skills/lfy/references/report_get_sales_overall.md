# get_sales_overall — 销售大局观（当前财年）

## 接口说明

查询**企业当前财年**内三条按日时间序列：

- `sum_actual`：实际签单金额（仅返回 `amount > 0` 的日期）。
- `sum_forecast`：预测签单金额（仅返回 `amount > 0` 的日期）。
- `total_opportunity`：商机池快照（Elasticsearch `b_pl_analysis_log` 按日聚合后，**仅当相对上一输出日的 `count` 或 `total_amount` 发生变化时才输出该日**，连日数值相同则合并省略，减少无效重复点）。

财年起止由服务端根据 `b_o_money` 当前财年记录自动解析，调用方无需传财年。

**权限**：需具备商机（`category_id=2`）**list** 或 **detail** 任一有效权限；否则返回 `error_message: 您暂无权限`。

## 稀疏返回怎么解读（重要）

接口为减轻体积**不返回完整日历**，解读规则如下（向用户或下游分析说明时使用）：

### `sum_actual` / `sum_forecast`

- 数组**为空** `[]`：在当前财年、当前过滤条件下，**没有任何一天**该项大于 0；等价于可理解为**覆盖范围内实际成交或预测成交金额「每天均为 0」**（无签单/无预测金额落在这条序列上）。
- 数组**非空**但**某个自然日 `date` 未出现在列表中**：该日**没有单独一条记录**，表示这一天的**实际签单金额**（`sum_actual`）或**预测签单金额**（`sum_forecast`）为 **0**（接口只返回 `amount > 0` 的日期）。

### `total_opportunity`

- 列表采用**变点压缩**：只输出 `(count, total_amount)` **相对上一输出日发生变化**的那一天。
- **某个自然日未出现在 `total_opportunity` 里**：通常表示从**上一输出日**到下一输出日之间，商机池的**总预测金额与条数相对前一日没有发生变化**（连日持平）；并非「没有商机」，而是**池子规模快照相对前一日无变化**，故不重复输出。
- **相邻两条输出**之间：中间所有未列出的日期，其含义与**前一条**的 `count`、`total_amount` 相同，直至下一条变点日。

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
