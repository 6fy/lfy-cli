# get_list — 联系人列表（分页查询）

## 命令

```bash
lfy-cli contact get_list '{"contacts_name":"","sales_ids":[],"page_size":20,"page":1}'
```

## 参数

| 参数名 | 类型 | 必填 | 默认 | 说明 |
| ------ | ---- | ---- | ---- | ---- |
| `contacts_name` | string | 否 | "" | 联系人姓名模糊搜索（ILIKE 不区分大小写，匹配姓+名拼接）；空串不加条件 |
| `sales_ids` | integer[] | 否 | `[]` | 负责人 ID 列表；`[]`=使用当前用户 list 权限白名单；非空=与白名单求交集 |
| `page_size` | integer | 否 | 20 | 每页数量，**1~100** |
| `page` | integer | 否 | 1 | 页码，从 1 开始 |

## 成功响应（lfy-cli-server 已剥离 code）

```jsonc
{
  "name": "联系人列表",
  "total": 10,
  "contacts": [
    {
      "contact_id": 1001,
      "contact_name": "张三",
      "sales_id": 111,
      "sales_name": "李销售",
      "phone": ["13800138000"],
      "email": ["zhang@example.com"],
      "customer_id": 200,
      "customer_name": "某某科技",
      "pipeline_id": 300,
      "pipeline_name": "年度采购项目",
      "jobtitle_id": 10,
      "jobtitle": "总监",
      "attitude_id": 20,
      "attitude": "支持",
      "create_time": "2025-10-10"
    }
  ]
}
```

### 字段说明

| 字段 | 类型 | 说明 |
|------|------|------|
| `name` | string | 固定 `"联系人列表"` |
| `total` | integer | 满足筛选条件的总数（分页前） |
| `contacts[]` | array | 当前页；无则 `[]` |
| `contact_id` | integer | 联系人 ID |
| `contact_name` | string | 姓名（lastname+firstname） |
| `sales_id` / `sales_name` | integer / string | 负责人 |
| `phone` / `email` | string[] | 激活的电话/邮箱，按 sort_order；无则 `[]` |
| `customer_id` / `customer_name` | integer / string | 关联表 category_id=1 且 createtime 最新的一条客户 |
| `pipeline_id` / `pipeline_name` | integer / string | 关联表 category_id=2 且 createtime 最新的一条商机 |
| `jobtitle_id` / `jobtitle` | integer / string | 职位枚举 |
| `attitude_id` / `attitude` | integer / string | 态度枚举 |
| `create_time` | string | 创建日期 `YYYY-MM-DD`，无则 `""` |

## 权限

- `per_user`：`category_id = 1544862211`，`category_scene = 'list'`
- 主查询：`b_c_contacts.sales_id` 须在 effective `sales_ids` 白名单内

## 错误处理

base_api 失败时返回 `{code:400, message:"...", data:null}`，lfy-cli-server 转为 `Error: GetList: {message}`。

- 无权限：`Error: GetList: 您暂无权限`
- 参数错误：`Error: GetList: 参数错误...`
- 远程失败：`Error: GetList: 远程服务错误，请稍后重试`
