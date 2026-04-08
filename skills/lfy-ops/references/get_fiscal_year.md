# get_fiscal_year — 获取当前财年信息

## 接口说明

获取企业当前财年的时间范围，根据企业设定的财年开始日期计算返回。日期按北京时间返回。

## 请求示例

```bash
lfy-cli ops get_fiscal_year '{}'
```

## 返回示例

```json
{
  "current_fiscal_year": 2026,
  "start_date": "2026-01-01",
  "end_date": "2026-12-31"
}
```

## 字段说明

| 字段                 | 类型   | 说明             |
| -------------------- | ------ | ---------------- |
| `current_fiscal_year` | integer | 当前财年年份     |
| `start_date`         | string | 财年开始日期，格式：YYYY-MM-DD |
| `end_date`           | string | 财年结束日期，格式：YYYY-MM-DD |
