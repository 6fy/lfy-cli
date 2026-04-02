---
name: lfy-customer
description: 客户查询技能，通过关键字模糊或精确搜索客户列表，返回客户ID、名称和GTM分类信息。
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli customer --help"
---

# 客户查询技能

> `lfy-cli` 是陆份仪提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过关键字搜索客户列表，支持模糊匹配和精确匹配。

## 操作

### 1. 搜索客户

使用 `customer search` 命令搜索客户：

**调用示例：**

```bash
lfy-cli customer search '{"keywords":"<keywords>"}'
```

**返回格式：**

```json
[
  {
    "customer_id": <customer_id_1>,
    "customer_name": "<customer_name_1>",
    "gtm_id": <gtm_id_1>
  },
  {
    "customer_id": <customer_id_2>,
    "customer_name": "<customer_name_2>",
    "gtm_id": <gtm_id_2>
  }
]
```

**返回字段说明：**

| 字段            | 类型    | 说明                      |
| --------------- | ------- | ------------------------- |
| `customer_id`   | integer | 客户唯一 ID               |
| `customer_name` | string  | 客户名称                  |
| `gtm_id`        | integer | GTM 分类 ID（业务线标识） |

---

### 2. 本地筛选策略

`search` 返回结果后，在本地对客户名称进行筛选匹配：

- **精确匹配**：`customer_name` 与关键词完全一致，直接使用
- **模糊匹配**：`customer_name` 包含关键词，返回所有匹配结果
- **无结果**：告知用户未找到对应客户

**搜索示例：**

用户问："帮我找一下<customer_name>这家客户"

1. 调用 `lfy-cli customer search '{"keywords":"<keywords>"}'`
2. 在结果中筛选 `customer_name` 包含"<customer_name>"的客户
3. 返回匹配结果

---

## 注意事项

- `search` 支持精确匹配和模糊匹配两种模式
- `keywords` 为空时可能返回错误或不完整结果
- `gtm_id` 是 GTM（Go To Market）业务线分类标识，用于区分不同业务线
- 若搜索结果为空，告知用户未找到对应客户
- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息

---

## 典型工作流

### 工作流 1：查询客户信息

用户问："帮我查一下<customer_name>是谁？"

1. 
```bash
lfy-cli customer search '{"keywords":"<keywords>"}'
```
 获取搜索结果

2. 在结果中筛选 `customer_name` 包含"<customer_name>"的客户
3. 若找到唯一匹配，直接展示结果：

```
📇 找到客户：
- 客户名称：<customer_name>
- 客户ID：<customer_id>
- GTM ID：<gtm_id>
```

4. 若找到多个匹配，展示候选列表请用户确认：

```
🔍 找到多个匹配客户，请确认您要查询的是哪家：

1. <customer_name_1>（ID：<customer_id_1>，GTM：<gtm_id_1>）
2. <customer_name_2>（ID：<customer_id_2>，GTM：<gtm_id_2>）

请问您要查询的是哪一家？
```

---

### 工作流 2：模糊搜索客户

用户问："帮我找一下包含'<keywords>'的客户"

1. 
```bash
lfy-cli customer search '{"keywords":"<keywords>"}'
```
 获取搜索结果

2. 在结果中筛选 `customer_name` 包含"<keywords>"的客户
3. 汇总后一并展示：

```
📇 找到 2 家客户：

1. <customer_name_1>
   - ID：<customer_id_1>
   - GTM ID：<gtm_id_1>

2. <customer_name_2>
   - ID：<customer_id_2>
   - GTM ID：<gtm_id_2>
```

---

## 快速参考

### 接口说明

| 接口              | 用途             | 输入            | 返回                                           |
| ----------------- | ---------------- | --------------- | ---------------------------------------------- |
| `customer search` | 按关键字搜索客户 | keywords 字符串 | 客户列表（customer_id、customer_name、gtm_id） |

### 筛选策略

| 场景                                           | 策略                     |
| ---------------------------------------------- | ------------------------ |
| 精确匹配（customer_name 完全一致）             | 直接使用，向用户展示结果 |
| 模糊匹配（customer_name 包含关键词），唯一结果 | 直接使用，向用户展示结果 |
| 模糊匹配，多个结果                             | 展示候选列表，请用户选择 |
| 无匹配结果                                     | 告知用户未找到对应客户   |
