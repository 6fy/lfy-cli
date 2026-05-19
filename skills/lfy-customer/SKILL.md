---
name: lfy-customer
description: 客户查询、创建与修改技能。当用户需要：(1) 按关键字搜索客户，(2) 获取 GTM 列表，(3) 客户详情，(4) 创建客户，(5) 修改客户字段，(6) 客户列表分页查询时使用此技能。
version: 1.4.0
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli customer --help"
---

# 客户技能

> `lfy-cli` 是陆份仪提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli customer <接口名> '<json入参>'` 与陆份仪平台的客户系统交互。

## 注意事项

- `keywords` 为空时可能返回错误或不完整结果
- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息
- 若搜索结果为空，告知用户未找到对应客户
- `gtm_id`, `customer_id` 等技术字段默认不展示
- **创建客户**需要 per_user 客户模块 **create** 场景权限；负责人默认本人，若指定 `sales_id` 须在 create 的 `sales_ids` 白名单内
- **修改客户**需要 **detail** 权限与销售范围（同 `get_details`）；成功 JSON 含 `code: 200`
- 访问客户详情页面：https://app.6fenyi.com/customers/{{customer_id}}

## 接口列表

### 搜索客户 (search)

```bash
lfy-cli customer search '{"keywords": "<keywords>"}'
```

按关键字搜索客户，支持模糊匹配。

参见 [API 详情](references/search.md)。

### 获取客户详情 (get_details)

```bash
lfy-cli customer get_details '{"customer_id": 123}'
```

获取指定客户的详细信息，包含客户主档、商机列表、联系人、跟进记录、近期相关任务（schedule）。需要客户详情权限。

参见 [API 详情](references/get_details.md)。

### 获取 GTM 列表 (get_gtms)

```bash
lfy-cli customer get_gtms '{}'
```

获取所有 GTM 业务线列表。

参见 [API 详情](references/get-gtms.md)。

### 创建客户 (create_customer)

```bash
lfy-cli customer create_customer '{"gtm_id": 1, "customer_name": "名称", "sales_id": 0}'
```

参见 [API 详情](references/create_customer.md)。

### 修改客户 (update_customer)

```bash
lfy-cli customer update_customer '{"customer_id": 123, "updates": {"customer_alias": "简称"}}'
```

参见 [API 详情](references/update_customer.md)。

### 客户列表 (get_list)

```bash
lfy-cli customer get_list '{"gtm_id":0,"customer_name":"","customer_status_ids":[],"sales_ids":[],"page_size":20,"page":1}'
```

分页查询当前用户 list 权限范围内的客户，支持按 GTM、名称（ILIKE 不区分大小写）、状态、销售人员过滤。响应为 `{name, total, customers}`（lfy-cli-server 已剥离 `code`）。

与 `search` 互补：`search` 用于轻量关键字搜 ID；`get_list` 用于带筛选、分页与业务指标的列表。

参见 [API 详情](references/get_list.md)。

---

## 典型工作流

### 搜索客户

**经典 query 示例：**
- "帮我搜索一下'科技'相关的客户"
- "找一下包含'未来'的客户"
- "搜索关键字为'成都'的客户有哪些？"
- "我在北京的客户有哪些？"

**流程：**
1. 提取用户提供的关键字
2. 调用 `search` 命令搜索客户
3. 在结果中筛选 `customer_name` 包含关键字的客户
4. 若找到唯一匹配，直接展示结果
5. 若找到多个匹配，最多展示前10个，并告知用户如果需要精准匹配请提供更具体的客户名称

**展示结果：**

找到客户时：

```
👥 为您找到 2 个客户： <customer_name_1>, <customer_name_2>
```

找不到客户时：

```
没有匹配到包含"<keywords>"的客户，请尝试更具体的方式问我，比如： "帮我搜索一下'科技'相关的客户"。
```

### 获取客户详情

**经典 query 示例：**
- "帮我看一下客户 123 的详细信息"
- "客户 456 的联系人和商机情况怎么样？"
- "查看这个客户的跟进记录"
- "客户 123 最近有什么任务？"

**流程：**
1. 获取 `customer_id`（可通过先搜索客户获得）
2. 调用 `get_details` 命令获取客户详情
3. 展示客户主档信息、商机列表、联系人、跟进记录、近期任务（schedule）

**展示结果：**

成功时：展示客户名称、销售负责人、状态、成熟度、标签等基本信息，以及商机列表（数量+名称）、联系人列表、近期跟进记录，以及近期任务（若 `schedule` 非空，展示任务条数及最接近今天的几条）。

无权限时：

```
Error: 您没有客户模块的权限
```

或

```
Error: 您没有访问此客户的权限
```

客户不存在时：

```
Error: 客户不存在
```

### 获取 GTM 列表

**经典 query 示例：**
- "GTM 业务线有哪些？"
- "帮我查一下 GTM 分类列表"
- "都有哪些 GTM？""

**流程：**
1. 调用 `get_gtms` 命令获取 GTM 列表
2. 展示 GTM 列表供用户查看

### 查询客户列表

**经典 query 示例：**

- "看一下我的客户列表"
- "搜一下名字带'科技'的客户，第 2 页"
- "X 销售负责的客户有哪些？"
- "状态为'意向'的客户第一页"

**流程：**

1. 若用户限定 GTM / 销售 / 状态，先通过对应技能拿 ID；否则相关字段保持 `0` 或 `[]`
2. `customer_name` trim 后空串则不传或传 `""`
3. `page_size` 默认 20，`page` 从 1 开始
4. 调用 `get_list`
5. 错误含「无权限」→ 告知用户无 list 权限
6. `total == 0` 或 `customers` 为空 → 告知「未匹配到客户」
7. 展示每条的 `customer_name`、`customer_status`、`sales_owner`、`gtm_name`、`annual_procurement_amount`、`pipeline_amount`、`last_interaction_time`、`tags`

**展示建议：**

👥 客户列表（共 total 条，当前第 page 页）：

| 客户 | 状态 | 负责人 | GTM | 年采购额 | 机会金额 | 最近互动 |
|------|------|--------|-----|----------|----------|----------|
| customer_name | customer_status | sales_owner | gtm_name | annual_procurement_amount | pipeline_amount | last_interaction_time |
