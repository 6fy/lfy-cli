---
name: lfy-pipeline
description: 商机技能。适用于按关键字搜索商机、查看详情、阶段配置、待签单列表、分页列表查询，以及在有权限时创建、修改商机。当用户需要搜索商机、查看详情/阶段、待签单或商机列表，或新建/修改一条商机时使用此技能。
version: 1.8.1
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli pipeline --help"
---

# 商机技能

> `lfy-cli` 是LFY提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli pipeline <方法名> '<json>'` 与商机系统交互。

## 注意事项

- `keywords` 为空时可能返回错误或不完整结果
- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息
- 若搜索结果为空，告知用户未找到对应商机
- `pipeline_id`、`stage_id` 等技术字段默认不展示
- **创建 / 修改**：`create` 需商机 create 权限；`update_pipeline` 需商机 detail 权限与 `sales_ids` 门禁；删除等其它操作仍不支持
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

### 获取最近待签单商机 (get_pending_signature)

```bash
lfy-cli pipeline get_pending_signature '{"gtm_id":0,"sales_ids":[],"customer_ids":[],"stage":0,"page_size":10,"page":1}'
```

查询当前用户 **list 权限范围内** 所有进行中的商机，按「距离签单近」的顺序排序。支持：

- `gtm_id`：按 GTM 过滤（0=全部）
- `sales_ids`：按销售人员过滤；`[]` 表示 list 权限全集，非空时会与白名单求交集，自动过滤超范围的 id，最多 50 个
- `customer_ids`：按客户过滤；`[]` 表示不过滤
- `stage`：阶段过滤（0=全部）
- 分页：`page`/`page_size`

参见 [API 详情](references/get_pending_signature.md)。

### 商机列表 (get_list)

```bash
lfy-cli pipeline get_list '{"gtm_id":0,"pipeline_name":"","pipeline_status_ids":[],"sales_ids":[],"page_size":20,"page":1}'
```

分页查询当前用户 list 权限范围内的商机，支持按 GTM、名称（ILIKE 不区分大小写）、状态、销售人员过滤。响应外层为 `{code, message, data:{name, total, pipelines}}`。

**展示结果**：必须使用 [HTML 模板](templates/get_list.html) 生成商机清单页面，写入临时文件后用系统浏览器打开（步骤见 [get_list HTML 报告](references/get_list_report.md)），不要在对话中贴大段 Markdown 表格。

参见 [API 详情](references/get_list.md)。

### 创建商机 (create)

```bash
lfy-cli pipeline create '{"gtm_id":17,"pipeline_name":"商机名称","customer_id":67,"phase_id":78,"sales_id":81,"forecast":9800,"forecast_date":"2026-07-12","tag_ids":[53]}'
```

在未掌握 `gtm_id`、`customer_id`、`phase_id` 等 ID 前，应先通过其它查询能力取得后再调用。

参见 [API 详情](references/create.md)。

### 修改商机 (update_pipeline)

```bash
lfy-cli pipeline update_pipeline '{"pipeline_id":111,"updates":{"projectname":"修改商机","forecast":100,"forecast_date":"2026-12-12","forecast_start_date":"2026-12-12","sales_id":1,"status_id":23123,"tags":"123,456","win_possibility":287705087406202,"actual":100,"stage_id":123}}'
```

- `pipeline_id`：必填，>0
- `updates`：必填；**仅出现的键会更新**；未放入 `updates` 的字段视为不修改；可为 `{}`（不写库，返回当前 `updated_time`）
- `actual` 须为合法数字（可为 0 或负数）；ID 类字段（`stage_id`、`sales_id`、`status_id`、`win_possibility`）须 >0，≤0 或非法报参数错误；`tags` 空串清空；日期字段空串清空

| 键 | 规则 | ID / 选项来源 |
|----|------|---------------|
| `projectname` | trim 后非空，≤50 字符 | — |
| `forecast` | ≥0，最多两位小数 | — |
| `forecast_date` | `YYYY-MM-DD`；`""` 清空 | — |
| `forecast_start_date` | `YYYY-MM-DD`；`""` 清空 | — |
| `sales_id` | >0；≤0 或非法报参数错误 | `lfy-cli user get_sales '{}'` |
| `status_id` | >0；≤0 或非法报参数错误 | `lfy-cli base get_options '{"object_id":<pipeline_id>,"property":"pipeline_status"}'` |
| `tags` | 逗号分隔标签 id；`""` 清空 | `lfy-cli base get_options '{"object_id":<pipeline_id>,"property":"pipeline_tags"}'` |
| `win_possibility` | >0；≤0 或非法报参数错误 | `lfy-cli base get_options '{"object_id":<pipeline_id>,"property":"pipeline_win_possibility"}'` |
| `actual` | 合法数字，最多两位小数（可为 0 或负数） | — |
| `stage_id` | >0，传 `get_sales_stage` 的 `stage_id`；≤0 或非法报参数错误 | `lfy-cli pipeline get_sales_stage '{"gtm_id":<gtm_id>}'` |

参见 [API 详情](references/update_pipeline.md)。

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

| 阶段名称     | 阶段类型值                                                      | 类型名称    | 里程碑目标       | 价值主张            | 建议天数                 |
| ------------ | --------------------------------------------------------------- | ----------- | ---------------- | ------------------- | ------------------------ |
| <stage_name> | <type_value>（10 线索 / 20 机会 / 30 交付 / 40 回款；无则为 0） | <type_name> | <milestone_goal> | <value_proposition> | <suggested_stage_days>天 |

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

### 查看最近待签单商机

**经典 query 示例：**

- "我最近有哪些待签单商机？"
- "列一下 80% 阶段的商机"
- "看一下我的下一批要签的单子"
- "X 销售最近要签哪些单？"（主管视角）
- "A 客户下有哪些要签的单？"

**流程：**

1. 若未指定阶段，`stage` 取 0；若提及「XX 阶段/XX%」映射为对应 `logic_phase`（如 80、90）
2. 若用户点名某销售（且在自己权限内），先通过 `lfy-user` 技能拿到销售 `user_id`，填入 `sales_ids`；未点名则保持 `[]`
3. 若用户点名某客户，先通过 `lfy-customer` 技能拿到 `customer_id`，填入 `customer_ids`；未点名则 `[]`
4. 若按 GTM，填入 `gtm_id`（可通过 `lfy-customer` 技能获取 GTM 列表）
5. `page_size` 默认 10；`page` 默认 1
6. 调用 `get_pending_signature`
7. `error_message == "您暂无权限"` → 告知用户无商机 list 权限
8. `total == 0` 或 `pipelines` 为空 → 告知「暂无符合条件的进行中待签单商机」
9. 展示每条的 `pipeline_name`、`customer_name`、`stage_name`（`stage_value`%）、`forecast_amount`（`forecast_set=false` 时标注「未填写预测金额」）、`stage_checklist` 进度

**展示建议：**

📌 最近待签单商机（共 total 条，当前第 page 页）：

| 商机          | 客户          | 阶段                      | 预测金额        | 行动清单                                       |
| ------------- | ------------- | ------------------------- | --------------- | ---------------------------------------------- |
| pipeline_name | customer_name | stage_name (stage_value%) | forecast_amount | completed_count/total_count（completion_rate） |

### 查询商机列表

**经典 query 示例：**

- "看一下我的商机列表"
- "搜一下名字带'科技'的商机，第 2 页"
- "X 销售名下的商机有哪些？"
- "状态为'进行中'的商机第一页"

**流程：**

1. 若用户限定 GTM / 销售 / 客户，先通过对应技能拿 ID；否则相关字段保持 `0` 或 `[]`
2. `pipeline_name` trim 后空串则不传或传 `""`
3. `page_size` 默认 20（与命令示例一致），`page` 从 1 开始
4. 调用 `get_list`
5. `error_message == "您暂无权限"` → 告知用户无 list 权限
6. `data.total == 0` 或 `data.pipelines` 为空 → 页眉注明「未匹配到商机」，`tbody` 可为空
7. **按 HTML 模板输出**（必须执行）：
   - 读取 `templates/get_list.html` 的版式与样式
   - 用 `data.total`、`page`、`pipelines` 填充页眉、状态徽章与表格行（字段映射见 [get_list_report.md](references/get_list_report.md)）
   - 写入临时 HTML 文件（如 `/tmp/lfy-pipeline-get_list-<时间戳>.html`）
   - **用浏览器打开**：macOS 执行 `open "<绝对路径>"`；Linux 执行 `xdg-open "<绝对路径>"`
8. 对话中仅简要说明：报告已在浏览器打开、共 total 条、当前第 page 页、文件路径；勿再贴 Markdown 大表

### 创建商机

**经典 query 示例：**

- 「帮我新建一个商机，名称 XX，挂在客户 YY 名下」
- 「创建一条商机，阶段是第一阶段」

**流程：**

1. 若缺失 `gtm_id`、`customer_id`、`phase_id` 等，先用客户/商机查询类能力取得 ID  
2. 拼装 JSON，调用 `create`  
3. `error_message`/CLI `Error` 中含「暂无权限」→ 说明无 create 或客户不在白名单  
4. 成功后展示返回的 `pipeline_id`、`pipeline_name`、`created_time`

### 修改商机

**经典 query 示例：**

- 「把商机 123 的名字改成 XX」
- 「这个商机阶段调到 80%」
- 「调整这条商机的预测金额到 98000」
- 「把这条商机的负责人改成销售 A」

**流程：**

1. 若用户只给了名称，先用 `search` 拿到 `pipeline_id`
2. 按待改字段查 ID：`stage_id` → `get_sales_stage` 取 `stage_id`；`sales_id` → `lfy-cli user get_sales`；`status_id` / `tags` / `win_possibility` → `lfy-cli base get_options`（`object_id` 填当前 `pipeline_id`，`property` 见上表或 [update_pipeline.md](references/update_pipeline.md)）
3. 仅把用户要改的字段放入 `updates`，调用 `update_pipeline`
4. `error_message`/CLI `Error` 含「您暂无权限」→ 说明无 detail 或 sales 不在白名单；含「商机不存在」→ 重新检查 `pipeline_id`
5. 成功后展示 `pipeline_id` 与 `updated_time`，并对已修改字段做回显确认
