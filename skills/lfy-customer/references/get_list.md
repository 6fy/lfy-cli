# get_list — 客户列表（分页查询）

## 命令

```bash
lfy-cli customer get_list '{"gtm_id":0,"customer_name":"","customer_status_ids":[],"sales_ids":[],"page_size":20,"page":1}'
```

## 参数

| 参数名 | 类型 | 必填 | 默认 | 说明 |
| ------ | ---- | ---- | ---- | ---- |
| `gtm_id` | integer | 否 | 0 | GTM ID；`0`=全部；>0 过滤 `c.gtm_id` |
| `customer_name` | string | 否 | "" | 客户名称模糊搜索（ILIKE 不区分大小写）；空串不加条件 |
| `customer_status_ids` | integer[] | 否 | `[]` | 客户状态 ID 列表；`[]`=不过滤；非空 `c.status_id = ANY` |
| `sales_ids` | integer[] | 否 | `[]` | 销售 ID 列表；`[]`=使用当前用户 list 权限白名单；非空=与白名单求交集 |
| `page_size` | integer | 否 | 20 | 每页数量，**1~100**；非法值由服务端拒绝 |
| `page` | integer | 否 | 1 | 页码，从 1 开始，<1 由服务端拒绝 |

## 成功响应（lfy-cli-server 已剥离 code）

```jsonc
{
  "name": "客户列表",
  "total": 21,
  "customers": [
    {
      "customer_id": 123,
      "customer_name": "客户名称",
      "customer_status": "客户状态",
      "customer_status_id": 123,
      "annual_procurement_amount": 1000,
      "pipeline_amount": 1000,
      "sales_id": 1111,
      "sales_owner": "销售负责人",
      "is_star": true,
      "tags": [{ "id": 123, "name": "A", "color": "#CCCCCC" }],
      "last_interaction_time": "2025-10-10",
      "gtm_id": 111,
      "gtm_name": "所属GTM",
      "annual_target": 1000,
      "customer_no": "xxx",
      "current_year_deals_amount": 1000,
      "current_year_remaining_pipeline": 1000,
      "pipeline_count": 10,
      "total_deals_amount": 1000,
      "create_time": "2025-10-10",
      "region": "区域",
      "region_id": 123,
      "industry": "行业",
      "industry_id": 123,
      "customer_alias": "别名",
      "created_by": "创建人",
      "created_by_id": 111,
      "gross_margin": 1000,
      "avg_collection_days": 17,
      "service_end_date": "2025-10-10"
    }
  ]
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | string | 固定 `"客户列表"` |
| `total` | integer | 满足筛选条件的总数（分页前） |
| `customers[]` | array | 当前页客户列表；无则 `[]` |
| `customers[].customer_id` | integer | 客户 ID |
| `customers[].customer_name` | string | 客户名称 |
| `customers[].customer_status` | string | 客户状态名称 |
| `customers[].customer_status_id` | integer | 客户状态 ID |
| `customers[].annual_procurement_amount` | number | 年采购金额 |
| `customers[].pipeline_amount` | number | 机会总金额 |
| `customers[].sales_id` | integer | 销售负责人 ID |
| `customers[].sales_owner` | string | 销售负责人姓名 |
| `customers[].is_star` | boolean | 是否星标 |
| `customers[].tags` | array | `[{id, name, color}]`；无标签 `[]` |
| `customers[].last_interaction_time` | string | 上次互动日期 `YYYY-MM-DD`，无则 `""` |
| `customers[].gtm_id` | integer | GTM ID |
| `customers[].gtm_name` | string | GTM 名称 |
| `customers[].annual_target` | number | 年销售目标 |
| `customers[].customer_no` | string | 客户编号/自编码 |
| `customers[].current_year_deals_amount` | number | 本财年已成交金额 |
| `customers[].current_year_remaining_pipeline` | number | 本财年剩余机会 |
| `customers[].pipeline_count` | integer | 商机数量 |
| `customers[].total_deals_amount` | number | 历史总成交金额 |
| `customers[].create_time` | string | 创建日期 `YYYY-MM-DD` |
| `customers[].region` | string | 区域名称 |
| `customers[].region_id` | integer | 区域 ID |
| `customers[].industry` | string | 行业名称 |
| `customers[].industry_id` | integer | 行业 ID |
| `customers[].customer_alias` | string | 客户别名 |
| `customers[].created_by` | string | 创建人姓名 |
| `customers[].created_by_id` | integer | 创建人 ID |
| `customers[].gross_margin` | number | 毛利率 |
| `customers[].avg_collection_days` | integer | 平均回款天数 |
| `customers[].service_end_date` | string | 服务结束日期 `YYYY-MM-DD`，无则 `""` |

## 与 search 的分工

| 场景 | 命令 |
|------|------|
| 按关键字快速找客户 ID | `customer search` |
| 带筛选/分页/业务指标的客户列表 | `customer get_list` |

## 权限

基于 `per_user` 表客户模块（`category_id=1`）**list** 场景的 `sales_ids` 白名单：

- 当前用户**无 list 权限** → CLI 报错含 `无权限`
- 有权限但白名单空 → `{total:0, customers:[]}`
- `sales_ids=[]`（缺省）→ 使用白名单全集
- `sales_ids=[1,2,3]` → 与白名单求交集后过滤；交集为空时 `total:0`

## 错误处理

| 文案 | 含义 |
| ---- | ---- |
| `无权限` | 无 list 权限或服务端异常兜底 |
| `page_size 不可为 0` | `page_size` 非法 |
| `page_size 须在 1～100 之间` | `page_size` 越界 |
| `page 要求大于 0` | `page<=0` |
| `参数错误：JSON 格式无效` | body 解析失败 |
