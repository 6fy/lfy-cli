# get_list — HTML 报告模板

展示 `get_list` 查询结果时，**必须**使用 HTML 模板生成可视化报告，并用系统默认浏览器打开，不再在对话中粘贴大段 Markdown 表格。

## 模板路径

```
lfy-cli/skills/lfy-pipeline/templates/get_list.html
```

技能安装后路径可能为 `~/.claude/skills/lfy-pipeline/templates/get_list.html`（以实际安装目录为准）。

## 生成步骤

1. 调用 `lfy-cli pipeline get_list '...'` 取得 JSON。
2. 读取 `templates/get_list.html` 作为版式参考（保留 `<style>` 与表头结构）。
3. 按下方字段映射填充页眉、状态徽章与 `<tbody>` 行。
4. 将完整 HTML 写入临时文件，文件名建议：`lfy-pipeline-get_list-<YYYYMMDD-HHMMSS>.html`。
5. **立即**用浏览器打开生成的文件（必须执行，不可仅告知路径）：

```bash
# macOS
open "/tmp/lfy-pipeline-get_list-20260519120000.html"

# Linux
xdg-open "/tmp/lfy-pipeline-get_list-20260519120000.html"
```

6. 在对话中简要说明：已生成商机清单报告、当前页/总数、报告文件路径；无数据时仍生成空表页并打开。

## 页眉

| 占位 | 来源 |
|------|------|
| 当前页条数 | `data.pipelines.length` |
| 页码 | 请求参数 `page` |
| 系统总计 | `data.total` |
| 已签单 / 进行中 / 已丢单 | 对**当前页** `pipelines[].status_name` 计数（全量统计需另请求或注明「当前页」） |

示例文案：`共 {n} 条（第 {page} 页） | 系统总计 {total} 条`

页脚示例：`生成时间：{北京时间 YYYY-MM-DD HH:mm:ss} · 第 {page} 页`

## 表格列与 API 字段

| 列 | 字段 | 说明 |
|----|------|------|
| # | 行号 | 从 1 起，`(page-1)*page_size + index` |
| 商机名称 | `pipeline_name` | 包在 `<strong>` 内 |
| 客户名称 | `customer_name` | 空则留空 |
| 客户简称 | `customer_alias` | |
| GTM | `gtm_name` | |
| 阶段 | `stage_name` | |
| 阶段% | `stage_value` | 格式 `{value}%` |
| 状态 | `status_name` | 见下方样式 |
| 预测金额 | `forecast` | 千分位，右对齐；0 可显示 `0` 或 `—` |
| 预计签单 | `forecast_date` | 空串显示 `—` |
| 负责人 | `owner_name` | |
| 最近互动 | `last_interaction_time` | |
| 创建时间 | `create_time` | |
| 标签 | `tags` | 无标签 `—`；有则 `<span class="tag">{name}</span>` |

## 状态单元格样式

状态列使用紧凑内联标签（勿给整格 `td` 上色），结构：

```html
<td class="col-status"><span class="status status-active">进行中</span></td>
```

| status_name | class |
|-------------|-------|
| 已签单 | `status-signed` |
| 进行中 | `status-active` |
| 已丢单 | `status-lost` |
| 其它 | `status-other` |

## 单行示例（Agent 生成时参考）

```html
<tr>
  <td style="text-align:center">1</td>
  <td><strong>示例商机</strong></td>
  <td>示例客户</td>
  <td>别名</td>
  <td>示例 GTM</td>
  <td>方案报价</td>
  <td style="text-align:center">80%</td>
  <td class="col-status"><span class="status status-active">进行中</span></td>
  <td class="forecast" style="text-align:right">67</td>
  <td>2026-07-05</td>
  <td>销售A</td>
  <td>2027-04-11</td>
  <td>2025-11-07</td>
  <td><span class="tag">重点</span></td>
</tr>
```

## 空结果

- `data.total == 0` 或 `pipelines` 为空：页眉写「共 0 条」，`tbody` 留空或一行提示「未匹配到商机」，仍执行 `open` 打开 HTML。
