---
name: lfy-ops
description: 运营数据查询技能。适用于获取企业财年时间范围、当前周数等运营数据。当用户需要：(1) 查询当前财年信息，(2) 获取当前属于第几周时使用此技能。
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli ops --help"
---

# 运营数据查询技能

> `lfy-cli` 是陆份仪提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli ops <接口名> '{}'` 与运营系统交互。

## 注意事项

- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息
- 财年信息由企业设定，日期按北京时间返回

---

## 接口列表

### get_fiscal_year — 获取当前财年信息

```bash
lfy-cli ops get_fiscal_year '{}'
```

获取企业当前财年的时间范围，根据企业设定的财年开始日期计算。

参见 [API 详情](references/get_fiscal_year.md)。

### get_current_week — 获取当前周数

```bash
lfy-cli ops get_current_week '{}'
```

获取当前属于第几周，按财年起始日期开始计算。

参见 [API 详情](references/get_current_week.md)。

---

## 返回格式

### get_fiscal_year 返回格式

```json
{
  "current_fiscal_year": 2026,
  "start_date": "2026-01-01",
  "end_date": "2026-12-31"
}
```

| 字段                 | 类型   | 说明             |
| -------------------- | ------ | ---------------- |
| `current_fiscal_year` | integer | 当前财年年份     |
| `start_date`         | string | 财年开始日期     |
| `end_date`           | string | 财年结束日期     |

### get_current_week 返回格式

```json
{
  "week_no": 5,
  "week_name": "W14",
  "start_date": "2026-03-30",
  "end_date": "2026-04-05"
}
```

| 字段        | 类型    | 说明             |
| ----------- | ------- | ---------------- |
| `week_no`   | integer | 第几周（从1开始） |
| `week_name` | string  | 周名称（如 W14）  |
| `start_date` | string  | 本周开始日期     |
| `end_date`  | string  | 本周结束日期     |

---

## 典型工作流

### 获取财年信息

**经典 query 示例：**
- "当前是第几个财年？"
- "财年什么时候开始？"
- "查一下今年的财年范围"

**流程：**
1. 调用 `get_fiscal_year` 命令获取财年信息
2. 展示财年开始和结束日期

**展示结果：**

📅 当前财年信息：

| 财年 | 开始日期 | 结束日期 |
|------|---------|---------|
| <current_fiscal_year>年 | <start_date> | <end_date> |

**字段映射**：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| current_fiscal_year | 财年 | 默认展示 |
| start_date | 开始日期 | 默认展示 |
| end_date | 结束日期 | 默认展示 |

---

### 获取当前周数

**经典 query 示例：**
- "现在是第几周？"
- "本周是几号到几号？"
- "当前周信息"

**流程：**
1. 调用 `get_current_week` 命令获取当前周信息
2. 展示当前周数及日期范围

**展示结果：**

📆 当前周信息：

| 周名称 | 开始日期 | 结束日期 |
|--------|---------|---------|
| <week_name> | <start_date> | <end_date> |

**字段映射**：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| week_name | 周名称 | 默认展示 |
| start_date | 开始日期 | 默认展示 |
| end_date | 结束日期 | 默认展示 |
| week_no | - | 技术字段，默认隐藏 |
