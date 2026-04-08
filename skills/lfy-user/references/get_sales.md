# get_sales — 获取销售人员名单

## 接口说明

获取所有销售人员及其所在的销售团队信息。返回团队层级结构和销售人员列表。

## 请求示例

```bash
lfy-cli user get_sales '{}'
```

## 返回示例

```json
{
  "teams": [
    {
      "team_id": 1111,
      "team_name": "销售一组",
      "parent_id": 1
    },
    {
      "team_id": 2222,
      "team_name": "销售二组",
      "parent_id": 1
    }
  ],
  "sales": [
    {
      "sales_id": 1,
      "sales_name": "张三",
      "teams": [1111]
    },
    {
      "sales_id": 2,
      "sales_name": "李四",
      "teams": [2222]
    }
  ]
}
```

## 字段说明

### teams 数组

| 字段         | 类型    | 说明           |
| ------------ | ------- | -------------- |
| `team_id`    | integer | 团队唯一 ID    |
| `team_name`  | string  | 团队名称       |
| `parent_id`  | integer | 上级团队 ID，根团队为 1 |

### sales 数组

| 字段        | 类型     | 说明             |
| ----------- | -------- | ---------------- |
| `sales_id`  | integer  | 销售人员唯一 ID |
| `sales_name` | string   | 销售人员姓名    |
| `teams`     | array    | 所属团队 ID 列表 |
