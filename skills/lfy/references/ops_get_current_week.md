# get_current_week — 获取当前周数

## 接口说明

获取当前属于第几周，按财年起始日期开始计算。日期按北京时间返回。

## 请求示例

```bash
lfy-cli ops get_current_week '{}'
```

## 返回示例

```json
{
  "week_no": 5,
  "week_name": "W14",
  "start_date": "2026-03-30",
  "end_date": "2026-04-05"
}
```

## 字段说明

| 字段        | 类型    | 说明                       |
| ----------- | ------- | -------------------------- |
| `week_no`   | integer | 第几周（从1开始计数）      |
| `week_name` | string  | 周名称，格式：W + 周序号   |
| `start_date` | string  | 本周开始日期，格式：YYYY-MM-DD |
| `end_date`  | string  | 本周结束日期，格式：YYYY-MM-DD |
