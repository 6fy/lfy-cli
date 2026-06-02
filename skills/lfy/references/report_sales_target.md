# sales_target — 获取销售财年合同目标

## 接口说明

查询指定 `sales_id` 在当前财年下的**合同目标**（`quota_type=1` 等业务含义由后端保证）。返回包含财年顶层信息，以及**固定键**的季度 `Q1`～`Q4`、月度 `M1`～`M12` 槽位；每个槽位带额度、起止日期及是否已在库中配置（`is_set`）。

无财年数据或缺少年维度行时，接口返回 `error_message`（如「暂未设置财年目标」），CLI 会以 `Error:` 形式展示。

## 请求示例

```bash
lfy-cli report sales_target '{"sales_id": 959782128042182}'
```

### 请求体字段

| 字段        | 类型   | 必填 | 说明 |
| ----------- | ------ | ---- | ---- |
| `sales_id`  | number | 是   | 组织内销售人员 ID，不必等于当前登录用户 ID |

## 返回示例（成功）

```json
{
  "sales_id": 959782128042182,
  "sales_name": "张三",
  "fiscal_year": 2026,
  "year_target": 2000000.0,
  "start_date": "2026-01-01",
  "end_date": "2026-12-31",
  "quarterly": {
    "Q1": { "val": 200000.0, "start": "2026-01-01", "end": "2026-03-31", "is_set": true },
    "Q2": { "val": 0, "start": "", "end": "", "is_set": false },
    "Q3": { "val": 400000.0, "start": "2026-07-01", "end": "2026-09-30", "is_set": true },
    "Q4": { "val": 1000000.0, "start": "2026-10-01", "end": "2026-12-31", "is_set": true }
  },
  "monthly": {
    "M1": { "val": 50000.0, "start": "2026-01-01", "end": "2026-01-31", "is_set": true },
    "M2": { "val": 0, "start": "", "end": "", "is_set": false }
  }
}
```

说明：`monthly` 中 **`M3`～`M12` 在真实响应里始终存在**（与 `Q1`～`Q4` 同理）；上表为篇幅仅示例前两月。

## 字段说明

### 顶层

| 字段           | 类型   | 说明 |
| -------------- | ------ | ---- |
| `sales_id`     | number | 请求中的销售 ID |
| `sales_name`   | string | 销售姓名 |
| `fiscal_year`  | number | 财年 |
| `year_target`  | number | 年维度目标额度 |
| `start_date`   | string | 财年起点，`YYYY-MM-DD` |
| `end_date`     | string | 财年终点，`YYYY-MM-DD` |

### quarterly / monthly 槽位（`SalesTargetSlot`）

键名固定：`quarterly` 为 `Q1`～`Q4`，`monthly` 为 `M1`～`M12`。

| 字段     | 类型    | 说明 |
| -------- | ------- | ---- |
| `val`    | number  | 目标额度；未配置槽位为 `0` |
| `start`  | string  | 该槽位在库中的开始日期，`YYYY-MM-DD`；**无行时为 `""`** |
| `end`    | string  | 该槽位在库中的结束日期，`YYYY-MM-DD`；**无行时为 `""`** |
| `is_set` | boolean | 库中存在对应季/月配置为 `true`（`val` 为 0 也可能为 `true`）；否则 `false` |

## 错误示例

```json
{
  "error_message": "暂未设置财年目标"
}
```

常见原因：当前无匹配财年 index、无年维度行、或该销售无配额数据等（与后端实现一致即可）。
