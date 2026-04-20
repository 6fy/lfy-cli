---
name: lfy-customer
description: 客户查询技能。适用于通过关键字搜索客户列表、获取客户 GTMs 分类、获取客户详情等需求。当用户需要：(1) 按关键字搜索客户，(2) 获取 GTM 业务线列表，(3) 获取客户详情时使用此技能。
version: 1.1.0
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli customer --help"
---

# 客户查询技能

> `lfy-cli` 是陆份仪提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli customer <接口名> '<json入参>'` 与陆份仪平台的客户系统交互。

## 注意事项

- `keywords` 为空时可能返回错误或不完整结果
- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息
- 若搜索结果为空，告知用户未找到对应客户
- `gtm_id`, `customer_id` 等技术字段默认不展示
- 当前版本不支持对客户进行任何修改操作
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
