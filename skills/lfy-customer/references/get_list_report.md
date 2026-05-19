# get_list — HTML 报告模板

展示 `customer get_list` 查询结果时，**必须**使用 HTML 模板生成可视化报告，并用系统默认浏览器打开，不再在对话中粘贴大段 Markdown 表格。

## 模板路径

```
lfy-cli/skills/lfy-customer/templates/get_list.html
```

技能安装后路径可能为 `~/.claude/skills/lfy-customer/templates/get_list.html`（以实际安装目录为准）。

## 生成步骤

1. 调用 `lfy-cli customer get_list '...'` 取得 JSON（顶层为 `{name, total, customers}`，无 `code` 包装）。
2. 读取 `templates/get_list.html` 作为版式参考（保留 `<style>` 与表头结构）。
3. 按下方字段映射填充页眉、状态徽章与 `<tbody>` 行。
4. 将完整 HTML 写入临时文件，文件名建议：`lfy-customer-get_list-<YYYYMMDD-HHMMSS>.html`。
5. **立即**用浏览器打开生成的文件（必须执行，不可仅告知路径）：

```bash
# macOS
open "/tmp/lfy-customer-get_list-20260519120000.html"

# Linux
xdg-open "/tmp/lfy-customer-get_list-20260519120000.html"
```

6. 在对话中简要说明：已生成客户清单报告、当前页/总数、报告文件路径；无数据时仍生成空表页并打开。

## 页眉

| 占位 | 来源 |
|------|------|
| 当前页条数 | `customers.length` |
| 页码 | 请求参数 `page` |
| 系统总计 | `total` |
| 状态徽章 | 对**当前页** `customers[].customer_status` 分组计数（名称以 API 返回为准） |

示例文案：`共 {n} 条（第 {page} 页） | 系统总计 {total} 条`

页脚示例：`生成时间：{北京时间 YYYY-MM-DD HH:mm:ss} · 第 {page} 页`

## 表格列与 API 字段

| 列 | 字段 | 说明 |
|----|------|------|
| # | 行号 | 从 1 起，`(page-1)*page_size + index` |
| 客户名称 | `customer_name` | 包在 `<strong>` 内；`is_star=true` 时在名前加 `<span class="star">★</span>` |
| 别名 | `customer_alias` | 空则留空 |
| 状态 | `customer_status` | 见下方样式 |
| 负责人 | `sales_owner` | |
| GTM | `gtm_name` | |
| 年采购额 | `annual_procurement_amount` | 千分位右对齐；0 或无效可 `—` |
| 机会金额 | `pipeline_amount` | 同上 |
| 商机数 | `pipeline_count` | 居中 |
| 区域 | `region` | |
| 行业 | `industry` | |
| 最近互动 | `last_interaction_time` | 空串 `—` |
| 标签 | `tags` | 无标签 `—`；有则 `<span class="tag">{name}</span>` |

## 状态单元格样式

使用紧凑内联标签（勿给整格 `td` 上色）：

```html
<td class="col-status"><span class="status status-intent">意向</span></td>
```

| customer_status（示例映射，以实际文案为准） | class |
|---------------------------------------------|-------|
| 含「意向」「潜在」 | `status-intent` |
| 含「成交」「签约」「合作」 | `status-won` |
| 含「沉默」「流失」「无效」 | `status-silent` |
| 其它 | `status-other` |

## 单行示例（Agent 生成时参考）

```html
<tr>
  <td style="text-align:center">1</td>
  <td><span class="star">★</span><strong>示例客户甲</strong></td>
  <td>客甲</td>
  <td class="col-status"><span class="status status-intent">意向</span></td>
  <td>销售A</td>
  <td>示例 GTM</td>
  <td class="amount">1,000,000</td>
  <td class="amount">500,000</td>
  <td style="text-align:center">3</td>
  <td>华东</td>
  <td>互联网</td>
  <td>2026-05-10</td>
  <td><span class="tag">重点</span></td>
</tr>
```

## 空结果

- `total == 0` 或 `customers` 为空：页眉写「共 0 条」，`tbody` 留空或一行提示「未匹配到客户」，仍执行 `open` 打开 HTML。
