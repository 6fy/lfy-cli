---
name: lfy-report
description: 报表查询技能。适用于通过 lfy-cli 的 report 品类读取陆份仪侧只读报表数据。当用户需要：(1) 查询指定销售人员当前财年的合同目标（年/季/月及是否已配置），(2) 查看当前财年销售大局观（实际/预测签单与商机池按日趋势），(3) 后续在 report 下扩展的其他只读报表接口时使用此技能；具体命令与参数以本技能 references 为准。
version: 1.1.2
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli report --help"
---

# 报表查询技能

> `lfy-cli` 是陆份仪提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli report <接口名> '<json参数>'` 与报表服务交互（需已完成 `lfy-cli init` 登录配置）。

## 注意事项

- 若命令输出以 `Error:` 开头或 JSON 中含 `error_message`，需向用户说明原因，勿伪造数据。
- 当前 **report** 品类下接口均为 **只读**，不支持通过本技能发起修改类操作。
- `sales_id` 等同 org 内技术 ID，面向业务用户展示时可优先展示 `sales_name` 等业务字段。
- 每新增一个 `report/<子命令>`，应在 `references/` 下增加对应文档，并在下方「接口列表」补充一节。
- **`get_sales_overall` 稀疏含义**（向用户解释时不要猜数）：
  - `sum_actual` / `sum_forecast` 为 **`[]`**：当前过滤下财年区间内该项**每天可视为 0**；若数组有数据但**缺某日**：该日**实际或预测签单金额为 0**（接口只返回 `amount > 0` 的日期）。
  - `total_opportunity` **缺某日**：多为**相对上一日池子总量与条数未变**（变点压缩）；两日之间的未列出日期与**上一输出点**的 `count`、`total_amount` 含义一致，直至下一条变点。详见 [get_sales_overall 文档](references/get_sales_overall.md#稀疏返回怎么解读重要)。

---

## 接口列表

### 销售财年合同目标 (sales_target)

```bash
lfy-cli report sales_target '{"sales_id": 123}'
```

查询指定销售在当前财年的**合同目标**（年目标 + 季/月槽位，含是否已配置）。

参见 [API 详情](references/sales_target.md)。

### 销售大局观 (get_sales_overall)

```bash
lfy-cli report get_sales_overall '{"gtm_id": 0, "sales_id": 0, "customer_ids": []}'
```

查询当前财年下的**实际签单**、**预测签单**、**商机池**三条按日时间序列；可按 GTM、销售、客户维度过滤（`0` / 空数组表示不过滤）。

参见 [API 详情](references/get_sales_overall.md)。

---

## 典型工作流

### 查询某销售的财年合同目标

**经典 query 示例：**

- 「查一下销售 ID 为 123 的本财年合同目标」
- 「张三的销售目标是多少」（需先具备或可解析出 `sales_id`）
- 「这个销售 Q2、M3 有没有设目标」
- 「我这个月的销售目标是多少」
- 「我这个季度的销售目标是多少」
- 「我这个财年的销售目标是多少」

**流程：**

1. 先确认销售人员的ID是否已知，已知的话直接进行第2步；否则先通过 `lfy-cli user get_sales '{}'` 获取销售人员名单，自动根据销售的名字匹配到对应的 sales_id，如果匹配到多个人名，需要让用户确认是哪个销售。
2. 执行 `lfy-cli report sales_target '{"sales_id": <id>}'`。
3. 若成功：结合返回中的 `year_target`、`quarterly`、`monthly` 与 `is_set` 向用户说明； `is_set` 为 `false` 时表示尚未配置该季度/月度的目标。
4. 若失败：根据终端 `Error:` 或 JSON `error_message` 说明（例如暂未设置财年目标）。

**展示结果（示例结构，非固定话术）：**

```
📊 销售目标（财年）：<sales_name>
年目标：<year_target>（<start_date> ~ <end_date>）
季度：Q1 … Q4（已配置项 is_set=true，未配置为 false）
月度：M1 … M12（同上）
```

### 分析销售大局观（结合目标）

**适用场景：** 需要「签单实际 / 预测 + 商机池 + 销售目标」一起做管理视角分析。

**流程：**

1. 明确维度：全员 / 某 GTM / 某销售 / 某客户；在 `get_sales_overall` 中设置 `gtm_id`、`sales_id`、`customer_ids`（未用的维度保持 `0` 或 `[]`）。
2. 执行 `lfy-cli report get_sales_overall '{...}'`，获取 `sum_actual`、`sum_forecast`、`total_opportunity`。解读稀疏数组时遵守上文「稀疏含义」：`sum_*` 缺日/空数组表示 0；`total_opportunity` 缺日表示池子相对前一日未变（变点序列）。
3. 当 **`sales_id` 非 0** 时，建议再执行 `lfy-cli report sales_target '{"sales_id": <同一 sales_id>}'`，将年/季/月目标与大局观曲线对照（目标完成度、预测与目标的缺口、商机池变化等）。
4. 当 **`sales_id` 为 0**（按 GTM、客户或全员看大盘）时，通常不调用 `sales_target`（目标接口面向单个销售），仅基于大局观数据解读即可。
5. 任一步若出现 `Error:` 或 `error_message`，如实告知用户，勿编造数值。
