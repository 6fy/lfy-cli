---
name: lfy-customer
description: 客户查询技能。适用于通过关键字搜索客户列表、获取客户 GTMs 分类等需求。当用户需要：(1) 按关键字搜索客户，(2) 获取 GTM 业务线列表时使用此技能。
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli customer --help"
---

# 客户查询技能

> `lfy-cli` 是陆份仪提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli customer <接口名> ' '` 与客户系统交互。

## 注意事项

- `keywords` 为空时可能返回错误或不完整结果
- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息
- 若搜索结果为空，告知用户未找到对应客户

---

## 接口列表

### search — 搜索客户

```bash
lfy-cli customer search '{"keywords": "<keywords>"}'
```

按关键字搜索客户，支持模糊匹配。

参见 [API 详情](references/search.md)。

### get_gtms — 获取 GTM 列表

```bash
lfy-cli customer get_gtms '{}'
```

获取所有 GTM 业务线列表。

参见 [API 详情](references/get-gtms.md)。

---

## 返回格式

### search 返回格式

```json
[
  {
    "customer_id": <customer_id>,
    "customer_name": "<customer_name>",
    "gtm_id": <gtm_id>
  }
]
```

| 字段           | 类型    | 说明                      |
| -------------- | ------- | ------------------------- |
| `customer_id`  | integer | 客户唯一 ID               |
| `customer_name` | string  | 客户名称                  |
| `gtm_id`       | integer | GTM 分类 ID（业务线标识） |

### get_gtms 返回格式

```json
[
  {
    "gtm_id": <gtm_id>,
    "gtm_name": "<gtm_name>"
  }
]
```

| 字段       | 类型    | 说明           |
| ---------- | ------- | -------------- |
| `gtm_id`   | integer | GTM 业务线 ID  |
| `gtm_name` | string  | GTM 业务线名称 |

---

## 典型工作流

### 搜索客户

**经典 query 示例：**
- "帮我搜索一下'科技'相关的客户"
- "找一下包含'未来'的客户"
- "搜索关键字为'成都'的客户有哪些？"

**流程：**
1. 提取用户提供的关键字
2. 调用 `search` 命令搜索客户
3. 在结果中筛选 `customer_name` 包含关键字的客户
4. 若找到唯一匹配，直接展示结果
5. 若找到多个匹配，展示候选列表请用户确认

**展示结果：**

找到客户时：
```
📇 找到客户：
- 客户名称：<customer_name>
- 客户ID：<customer_id>
- GTM ID：<gtm_id>
```

多个匹配时：
```
🔍 找到多个匹配客户，请确认您要查询的是哪家：

1. <customer_name_1>（ID：<customer_id_1>，GTM：<gtm_id_1>）
2. <customer_name_2>（ID：<customer_id_2>，GTM：<gtm_id_2>）

请问您要查询的是哪一家？
```

未找到时：
```
未找到包含"<keywords>"的客户，请尝试其他关键字。
```

---

### 获取 GTM 列表

**经典 query 示例：**
- "GTM 业务线有哪些？"
- "帮我查一下 GTM 分类列表"
- "都有哪些 GTM？""

**流程：**
1. 调用 `get_gtms` 命令获取 GTM 列表
2. 展示 GTM 列表供用户查看

**展示结果：**
```
📋 GTM 业务线列表：

1. 华北区（ID：123）
2. 华东区（ID：124）
3. 华南区（ID：125）
```
