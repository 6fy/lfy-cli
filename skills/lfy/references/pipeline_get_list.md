# get_list — 商机列表（分页查询）

## 命令

```bash
lfy-cli pipeline get_list '{"gtm_id":0,"pipeline_name":"","pipeline_status_ids":[],"sales_ids":[],"page_size":20,"page":1}'
```

## 参数

| 参数名 | 类型 | 必填 | 默认 | 说明 |
| ------ | ---- | ---- | ---- | ---- |
| `gtm_id` | integer | 否 | 0 | GTM ID；`0`=全部；>0 过滤 `p.gtm_id` |
| `pipeline_name` | string | 否 | "" | 商机名称模糊搜索（ILIKE 不区分大小写）；空串不加条件 |
| `pipeline_status_ids` | integer[] | 否 | `[]` | 商机状态 ID 列表；`[]`=不过滤；非空 `p.status_id = ANY` |
| `sales_ids` | integer[] | 否 | `[]` | 销售 ID 列表；`[]`=使用当前用户 list 权限白名单；非空=与白名单求交集（自动过滤不在范围内的 id） |
| `page_size` | integer | 否 | 20 | 每页数量，**1~100**；非法值由服务端拒绝 |
| `page` | integer | 否 | 1 | 页码，从 1 开始，<1 由服务端拒绝 |

## 成功响应（双层包装）

```jsonc
{
  "code": 200,
  "message": "success",
  "data": {
    "name": "商机列表",
    "total": 74,
    "pipelines": [
      {
        "pipeline_id": 17,
        "pipeline_name": "示例商机",
        "customer_id": 96,
        "customer_name": "示例客户",
        "customer_alias": "别名",
        "sales_id": 86,
        "owner_name": "冯沐辰",
        "gtm_id": 88,
        "gtm_name": "示例 GTM",
        "self_code_v2": "48",
        "stage_id": 80,
        "stage_name": "方案报价",
        "stage_value": 80,
        "status_id": 21,
        "status_name": "进行中",
        "relationship_level_id": 25,
        "relationship_level_name": "高",
        "win_possibility_id": 62,
        "win_possibility_name": "70%",
        "biz_type_id": 73,
        "biz_type_name": "新签",
        "forecast": 67.00,
        "actual_date": "",
        "forecast_date": "2026-07-05",
        "start_time": "2025-11-16",
        "create_time": "2025-11-07",
        "last_interaction_time": "2027-04-11",
        "tags": [
          { "id": 90, "name": "重点", "color": "#FFFFFF" }
        ]
      }
    ]
  }
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `code` | integer | 固定 `200` |
| `message` | string | 固定 `"success"` |
| `data.name` | string | 固定 `"商机列表"` |
| `data.total` | integer | 满足筛选条件的总数（分页前） |
| `data.pipelines[]` | array | 当前页商机列表；无则 `[]` |
| `data.pipelines[].pipeline_id` | integer | 商机 ID |
| `data.pipelines[].pipeline_name` | string | 商机名称 |
| `data.pipelines[].customer_id` | integer | 关联客户 ID |
| `data.pipelines[].customer_name` | string | 客户名称 |
| `data.pipelines[].customer_alias` | string | 客户别名 |
| `data.pipelines[].sales_id` | integer | 销售负责人 ID |
| `data.pipelines[].owner_name` | string | 销售负责人姓名 |
| `data.pipelines[].gtm_id` | integer | GTM ID |
| `data.pipelines[].gtm_name` | string | GTM 名称 |
| `data.pipelines[].self_code_v2` | string | 商机自编码 |
| `data.pipelines[].stage_id` | integer | 阶段 ID |
| `data.pipelines[].stage_name` | string | 阶段名称 |
| `data.pipelines[].stage_value` | integer | 阶段百分比（10/20/…/100） |
| `data.pipelines[].status_id` | integer | 状态 ID |
| `data.pipelines[].status_name` | string | 状态名称 |
| `data.pipelines[].relationship_level_id` | integer | 客户成熟度 ID |
| `data.pipelines[].relationship_level_name` | string | 客户成熟度名称 |
| `data.pipelines[].win_possibility_id` | integer | 签单可能性 ID |
| `data.pipelines[].win_possibility_name` | string | 签单可能性名称 |
| `data.pipelines[].biz_type_id` | integer | 业务分类 ID |
| `data.pipelines[].biz_type_name` | string | 业务分类名称 |
| `data.pipelines[].forecast` | number | 预测金额；NULL → 0 |
| `data.pipelines[].actual_date` | string | 实际签单日期 `YYYY-MM-DD`，无则 "" |
| `data.pipelines[].forecast_date` | string | 预测签单日期 `YYYY-MM-DD`，无则 "" |
| `data.pipelines[].start_time` | string | 启动日期 `YYYY-MM-DD`，无则 "" |
| `data.pipelines[].create_time` | string | 创建日期 `YYYY-MM-DD`（北京时间） |
| `data.pipelines[].last_interaction_time` | string | 最近修改日期 `YYYY-MM-DD`（北京时间） |
| `data.pipelines[].tags` | array | `[{id, name, color}]`；无标签返回 `[]` |

## 权限

基于 `per_user` 表商机模块（`category_id=2`）**list** 场景的 `sales_ids` 白名单：

- 当前用户**无 list 权限**（per_user 无行 / `scene_scope=0` / NULL）→ `{"error_message":"您暂无权限"}`
- 有权限但白名单空 → `{data:{total:0, pipelines:[]}}`
- `sales_ids=[]`（缺省）→ 使用白名单全集
- `sales_ids=[1,2,3]` → 与白名单求交集后 `p.sales_id = ANY(交集)`；交集为空时 `total:0`

## 错误处理

| 文案 | 含义 |
| ---- | ---- |
| `您暂无权限` | 参数非法 / 无 list 权限 / 服务端异常的统一兜底 |
| `参数错误：page 须大于 0` | `page<=0` |
| `参数错误：page_size 须在 1-100 之间` | `page_size` 非法 |
| `参数错误` | body JSON 解析失败 |
