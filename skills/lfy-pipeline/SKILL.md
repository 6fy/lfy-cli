---
name: lfy-pipeline
description: 商机查询技能。适用于按关键字搜索商机列表、按 pipeline_id 获取商机详情、按 gtm 拉取阶段配置。当用户需要搜索商机、查看某条商机详情或阶段信息时使用此技能。
version: 1.1.0
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli pipeline --help"
---

# 商机查询技能

> `lfy-cli` 是LFY提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli pipeline <接口名> ' '` 与商机系统交互。

## 注意事项

- `keywords` 为空时可能返回错误或不完整结果
- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息
- 若搜索结果为空，告知用户未找到对应商机
- `pipeline_id`、`stage_id` 等技术字段默认不展示
- 当前版本不支持对商机进行任何修改操作
- 访问商机详情页面：https://app.6fenyi.com/pipelines/{{pipeline_id}}

## 接口列表

### 搜索商机 (search)

```bash
lfy-cli pipeline search '{"keywords": "<keywords>"}'
```

按关键字搜索商机，支持模糊匹配。

参见 [API 详情](references/search.md)。

### 获取商机阶段 (get_sales_stage)

```bash
lfy-cli pipeline get_sales_stage '{"gtm_id": <gtm_id>}'
```

根据 GTM ID 获取商机阶段列表，包括阶段名称、里程碑目标、价值主张等信息。

参见 [API 详情](references/get_sales_stage.md)。

### 获取商机详情 (get_pipeline_info)

```bash
lfy-cli pipeline get_pipeline_info '{"pipeline_id": <pipeline_id>}'
```

根据商机 ID 获取详情（主档、推荐周期、当前阶段、商机侧与客户侧联系人、销售阶段全景与每阶段的推荐任务、商机相关近期任务等）。需具备商机模块 **detail** 权限且负责人在可见 `sales_ids` 范围内。

参见 [API 详情](references/get_pipeline_info.md)。

---

## 典型工作流

### 搜索商机

**经典 query 示例：**
- "帮我搜索一下'科技'相关的商机"
- "找一下包含'未来'的商机"
- "搜索关键字为'成都'的商机有哪些？"

**流程：**
1. 提取用户提供的关键字
2. 调用 `search` 命令搜索商机
3. 在结果中筛选 `pipeline_name` 包含关键字的商机
4. 若找到唯一匹配，直接展示结果
5. 若找到多个匹配，最多展示前10个，并告知用户如果需要精准匹配请提供更具体的商机名称

**展示结果：**

📇 为您找到 1 个商机：<pipeline_name>

找不到时：

```
没有匹配到包含"<keywords>"的商机，请尝试更具体的方式问我，比如："帮我搜索一下'科技'相关的商机"。
```

### 获取商机阶段

**经典 query 示例：**
- "帮我查一下这个商机的阶段信息"
- "获取商机阶段"
- "这个商机进行到哪一步了"

**流程：**
1. 提取用户提供的 `gtm_id`
2. 调用 `get_sales_stage` 命令获取阶段信息
3. 展示阶段列表信息

**展示结果：**

📋 商机阶段信息：

| 阶段名称 | 里程碑目标 | 价值主张 | 建议天数 |
|----------|-----------|---------|---------|
| <stage_name> | <milestone_goal> | <value_proposition> | <suggested_stage_days>天 |

### 获取商机详情

**经典 query 示例：**
- "查一下商机 123 的详情"
- "这个 pipeline 的联系人、阶段、预测金额是什么"
- "这个商机进行到哪个阶段了？最近有哪些任务？"

**流程：**
1. 若只有名称没有 ID，先用 `search` 得到 `pipeline_id`
2. 调用 `get_pipeline_info`，传入 `pipeline_id`
3. 将 `current_sales_stage`、`sales_stages`、`schedule`、`pipeline_contacts`、`customer_contacts` 等按用户问题整理展示；无阶段时说明 `current_sales_stage` 为空；`sales_stages` / `schedule` 为 `[]` 时明确告知「该商机暂无阶段配置 / 暂无近期任务」

**错误时：** 根据返回的 `error_message` 原文告知用户（如「商机不存在」「您没有访问此商机的权限」）。
