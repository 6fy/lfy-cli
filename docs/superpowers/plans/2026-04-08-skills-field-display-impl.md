# Skills 字段展示规范实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 为 5 个 SKILL.md 文件添加字段展示模板，规范技术字段隐藏规则，使用中文表格形式展示数据。

**Architecture:** 在每个 SKILL.md 的"典型工作流"章节中，为每个接口增加"展示模板"小节，包含字段映射表。遵循设计规范：技术字段默认隐藏，英文字段名不直接展示，表格形式展示。

**Tech Stack:** Markdown 文档修改

---

## 文件修改概览

| 文件 | 接口数 | 需隐藏字段 |
|------|--------|-----------|
| `skills/lfy-user/SKILL.md` | 2 | `user_id`, `team_id`, `parent_id`, `sales_id`, `teams` |
| `skills/lfy-customer/SKILL.md` | 2 | `customer_id`, `gtm_id` |
| `skills/lfy-pipeline/SKILL.md` | 2 | `pipeline_id`, `stage_id` |
| `skills/lfy-schedule/SKILL.md` | 1 | `task_id`, `task_type`, `status_value` |
| `skills/lfy-ops/SKILL.md` | 2 | `week_no` |

---

## Task 1: 修改 lfy-user/SKILL.md

**Files:**
- Modify: `skills/lfy-user/SKILL.md:109-166`

### get_self 接口

**现有展示结果（需替换）：**
```markdown
👤 当前用户信息：
- 用户ID：<user_id>
- 用户名：<user_name>
- 组织：<org_name>
```

**修改为：**

```markdown
👤 当前用户信息：

| 姓名 | 组织名称 |
|------|----------|
| <user_name> | <org_name> |

**字段映射**：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| user_name | 姓名 | 默认展示 |
| org_name | 组织名称 | 默认展示 |
| user_id | - | 技术字段，默认隐藏 |
```

### get_sales 接口

**现有展示结果（需替换）：**
```markdown
👥 销售人员列表：

**销售一组**
- 张三（ID：1）

**销售二组**
- 李四（ID：2）
- 王五（ID：3）
```

**修改为：**

```markdown
👥 销售人员列表：

**销售一组**
- 张三

**销售二组**
- 李四
- 王五

**字段映射**：

teams 数组：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| team_name | 团队名称 | 默认展示 |
| team_id | - | 技术字段，默认隐藏 |
| parent_id | - | 技术字段，默认隐藏 |

sales 数组：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| sales_name | 姓名 | 默认展示 |
| sales_id | - | 技术字段，默认隐藏 |
| teams | - | 技术字段，默认隐藏 |
```

---

## Task 2: 修改 lfy-customer/SKILL.md

**Files:**
- Modify: `skills/lfy-customer/SKILL.md:86-148`

### search 接口

**现有展示结果（需替换）：**
```markdown
📇 找到客户：
- 客户名称：<customer_name>
- 客户ID：<customer_id>
- GTM ID：<gtm_id>

多个匹配时：
🔍 找到多个匹配客户，请确认您要查询的是哪家：

1. <customer_name_1>（ID：<customer_id_1>，GTM：<gtm_id_1>）
2. <customer_name_2>（ID：<customer_id_2>，GTM：<gtm_id_2>）
```

**修改为：**

```markdown
📇 找到客户：

| 客户名称 |
|----------|
| <customer_name> |

**字段映射**：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| customer_name | 客户名称 | 默认展示 |
| customer_id | - | 技术字段，默认隐藏 |
| gtm_id | - | 技术字段，默认隐藏 |

多个匹配时：
🔍 找到多个匹配客户，请确认您要查询的是哪家：

| 客户名称 |
|----------|
| <customer_name_1> |
| <customer_name_2> |
```

### get_gtms 接口

**现有展示结果（需替换）：**
```markdown
📋 GTM 业务线列表：

1. 华北区（ID：123）
2. 华东区（ID：124）
3. 华南区（ID：125）
```

**修改为：**

```markdown
📋 GTM 业务线列表：

| GTM业务线 |
|-----------|
| 华北区 |
| 华东区 |
| 华南区 |

**字段映射**：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| gtm_name | GTM业务线 | 默认展示 |
| gtm_id | - | 技术字段，默认隐藏 |
```

---

## Task 3: 修改 lfy-pipeline/SKILL.md

**Files:**
- Modify: `skills/lfy-pipeline/SKILL.md:94-154`

### search 接口

**现有展示结果（需替换）：**
```markdown
📇 找到商机：
- 商机名称：<pipeline_name>
- 商机ID：<pipeline_id>

多个匹配时：
🔍 找到多个匹配商机，请确认您要查询的是哪个：

1. <pipeline_name_1>（ID：<pipeline_id_1>）
2. <pipeline_name_2>（ID：<pipeline_id_2>）
```

**修改为：**

```markdown
📇 找到商机：

| 商机名称 |
|----------|
| <pipeline_name> |

**字段映射**：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| pipeline_name | 商机名称 | 默认展示 |
| pipeline_id | - | 技术字段，默认隐藏 |

多个匹配时：
🔍 找到多个匹配商机，请确认您要查询的是哪个：

| 商机名称 |
|----------|
| <pipeline_name_1> |
| <pipeline_name_2> |
```

### get_sales_stage 接口

**现有展示结果（需替换）：**
```markdown
📋 商机阶段信息：
- 阶段名称：<stage_name>
- 里程碑目标：<milestone_goal>
- 价值主张：<value_proposition>
- 建议天数：<suggested_stage_days>天
```

**修改为：**

```markdown
📋 商机阶段信息：

| 阶段名称 | 里程碑目标 | 价值主张 | 建议天数 |
|----------|-----------|---------|---------|
| <stage_name> | <milestone_goal> | <value_proposition> | <suggested_stage_days>天 |

**字段映射**：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| stage_name | 阶段名称 | 默认展示 |
| milestone_goal | 里程碑目标 | 默认展示 |
| value_proposition | 价值主张 | 默认展示 |
| suggested_stage_days | 建议天数 | 默认展示 |
| stage_id | - | 技术字段，默认隐藏 |
```

---

## Task 4: 修改 lfy-schedule/SKILL.md

**Files:**
- Modify: `skills/lfy-schedule/SKILL.md:92-131`

### get_recent_tasks 接口

**现有展示结果（需替换）：**
```markdown
📋 最近日程任务（共<count>项）：

**进行中**
1. [任务] 完成官方文档翻译
   - 编号：TASK-2024-001
   - 时间：2024-03-01 09:00 至 2024-03-05 18:00
   - 负责人：张三
   - 优先级：高
```

**修改为：**

```markdown
📋 最近日程任务（共<count>项）：

**进行中**

| 编号 | 任务名称 | 类型 | 开始时间 | 截止时间 | 负责人 | 状态 | 优先级 |
|------|---------|------|---------|---------|-------|------|--------|
| TASK-2024-001 | 完成官方文档翻译 | 任务 | 2024-03-01 09:00 | 2024-03-05 18:00 | 张三 | 进行中 | 高 |

**字段映射**：

| 原始字段 | 中文表头 | 备注 |
|---------|---------|------|
| task_no | 编号 | 默认展示 |
| task_name | 任务名称 | 默认展示 |
| type_name | 类型 | 默认展示 |
| start_time | 开始时间 | 默认展示 |
| due_time | 截止时间 | 默认展示 |
| owner_name | 负责人 | 默认展示 |
| status_name | 状态 | 默认展示 |
| priority_name | 优先级 | 默认展示 |
| task_id | - | 技术字段，默认隐藏 |
| task_type | - | 技术字段，默认隐藏 |
| status_value | - | 技术字段，默认隐藏 |
```

---

## Task 5: 修改 lfy-ops/SKILL.md

**Files:**
- Modify: `skills/lfy-ops/SKILL.md:85-126`

### get_fiscal_year 接口

**现有展示结果（需替换）：**
```markdown
📅 当前财年信息：
- 财年：<current_fiscal_year>年
- 开始日期：<start_date>
- 结束日期：<end_date>
```

**修改为：**

```markdown
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
```

### get_current_week 接口

**现有展示结果（需替换）：**
```markdown
📆 当前周信息：
- 第<week_no>周（<week_name>）
- 开始日期：<start_date>
- 结束日期：<end_date>
```

**修改为：**

```markdown
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
```

---

## Task 6: 提交所有更改

**Files:**
- Modify: `skills/lfy-user/SKILL.md`
- Modify: `skills/lfy-customer/SKILL.md`
- Modify: `skills/lfy-pipeline/SKILL.md`
- Modify: `skills/lfy-schedule/SKILL.md`
- Modify: `skills/lfy-ops/SKILL.md`

```bash
git add skills/lfy-user/SKILL.md skills/lfy-customer/SKILL.md skills/lfy-pipeline/SKILL.md skills/lfy-schedule/SKILL.md skills/lfy-ops/SKILL.md
git commit -m "$(cat <<'EOF'
feat: 为所有 SKILL.md 添加字段展示模板

- 技术字段默认隐藏（id, _id, _at 结尾的字段）
- 使用中文表格形式展示数据
- 添加字段映射说明
- 遵循业务用户友好原则
EOF
)"
```

---

## 实施顺序

1. Task 1: 修改 lfy-user/SKILL.md
2. Task 2: 修改 lfy-customer/SKILL.md
3. Task 3: 修改 lfy-pipeline/SKILL.md
4. Task 4: 修改 lfy-schedule/SKILL.md
5. Task 5: 修改 lfy-ops/SKILL.md
6. Task 6: 提交所有更改
