# 客户技能

> `lfy-cli` 是LFY提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli customer <接口名> '<json入参>'` 与LFY平台的客户系统交互。

## 注意事项

- `keywords` 为空时可能返回错误或不完整结果
- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息
- 若搜索结果为空，告知用户未找到对应客户
- `gtm_id`, `customer_id` 等技术字段默认不展示
- **创建客户**需要 per_user 客户模块 **create** 场景权限；负责人默认本人，若指定 `sales_id` 须在 create 的 `sales_ids` 白名单内
- **修改客户**需要 **detail** 权限与销售范围（同 `get_details`）；成功 JSON 含 `code: 200`
- 访问客户详情页面：https://app.6fenyi.com/customers/{{customer_id}}

## search 与 get_list 如何选

| 用户意图 | 使用接口 | 说明 |
| -------- | -------- | ---- |
| 我的客户列表 / 我的客户清单 / LFY 我的客户清单 | **`get_list`** | 无明确「搜索关键字」、要看权限内分页清单时**必须**走此接口 |
| 列出/查看我负责的客户、全部客户一览 | **`get_list`** | `sales_ids` 默认 `[]` 即 list 权限范围 |
| 名字带「XX」的客户（可带分页） | **`get_list`** | `customer_name` 填关键字 |
| 按 GTM / 状态 / 某销售筛选客户 | **`get_list`** | 对应筛选参数 |
| 按关键字快速找客户 ID（为详情/修改铺路） | **`search`** | 轻量模糊搜，结果少、无业务指标 |
| 明确说「搜索」「找一下包含 XX」 | **`search`** | 用户强调搜索动作且无列表/清单语义时 |

**禁止**：用户问「我的客户列表」「我的客户清单」「LFY 我的客户清单」等时调用 `search`。

## 接口列表

### 搜索客户 (search)

```bash
lfy-cli customer search '{"keywords": "<keywords>"}'
```

按关键字搜索客户，支持模糊匹配。**不用于**「我的客户列表/清单」类需求（见上文路由表，应使用 `get_list`）。

参见 [API 详情](customer_search.md)。

### 获取客户详情 (get_details)

```bash
lfy-cli customer get_details '{"customer_id": 123}'
```

获取指定客户的详细信息，包含客户主档、商机列表、联系人、跟进记录、近期相关任务（schedule）。需要客户详情权限。

参见 [API 详情](customer_get_details.md)。

### 获取 GTM 列表 (get_gtms)

```bash
lfy-cli customer get_gtms '{}'
```

获取所有 GTM 列表。

参见 [API 详情](customer_get_gtms.md)。

### 创建客户 (create_customer)

```bash
lfy-cli customer create_customer '{"gtm_id": 1, "customer_name": "名称", "sales_id": 0}'
```

参见 [API 详情](customer_create.md)。

### 修改客户 (update_customer)

```bash
lfy-cli customer update_customer '{"customer_id": 123, "updates": {"customer_alias": "简称"}}'
```

参见 [API 详情](customer_update.md)。

### 客户列表 (get_list)

```bash
lfy-cli customer get_list '{"gtm_id":0,"customer_name":"","customer_status_ids":[],"sales_ids":[],"page_size":20,"page":1}'
```

分页查询当前用户 list 权限范围内的客户，支持按 GTM、名称（ILIKE 不区分大小写）、状态、销售人员过滤。响应为 `{name, total, customers}`（lfy-cli-server 已剥离 `code`）。

与 `search` 互补：`search` 用于轻量关键字搜 ID；`get_list` 用于带筛选、分页与业务指标的列表。

**展示结果**：必须使用 [HTML 模板](../templates/customer_get_list.html) 生成客户清单页面，写入临时文件后用系统浏览器打开（步骤见 [get_list HTML 报告](customer_get_list_report.md)），不要在对话中贴大段 Markdown 表格。

参见 [API 详情](customer_get_list.md)。

---

## 典型工作流

### 搜索客户

**适用条件：** 用户明确要**按关键字搜索**找客户，且**不是**要查看「我的客户列表/清单」。若用户要列表/清单，改走下方「查询客户列表」并使用 `get_list`。

**经典 query 示例：**
- "帮我搜索一下'科技'相关的客户"
- "找一下包含'未来'的客户"
- "搜索关键字为'成都'的客户"

**不适用（应走 `get_list`）：**
- "我的客户列表" / "我的客户清单" / "LFY 我的客户清单"
- "看一下我有哪些客户" / "列出我的客户"

**流程：**
1. 先判断：若属于列表/清单类意图 → **不要**调用 `search`，改走 `get_list` 工作流
2. 提取用户提供的关键字
3. 调用 `search` 命令搜索客户
4. 在结果中筛选 `customer_name` 包含关键字的客户
5. 若找到唯一匹配，直接展示结果
6. 若找到多个匹配，最多展示前10个，并告知用户如果需要精准匹配请提供更具体的客户名称

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
- "GTM 有哪些？"
- "帮我查一下 GTM 列表"
- "都有哪些 GTM？""

**流程：**
1. 调用 `get_gtms` 命令获取 GTM 列表
2. 展示 GTM 列表供用户查看

### 查询客户列表

**适用条件：** 用户要查看**客户清单/列表**（含「我的」），或带筛选/分页的列表查询。**默认接口**，优先于 `search`。

**经典 query 示例：**

- "看一下我的客户列表"
- "我的客户清单"
- "LFY 我的客户清单"
- "我有哪些客户"
- "列出我负责的客户"
- "搜一下名字带'科技'的客户，第 2 页"
- "X 销售负责的客户有哪些？"
- "状态为'意向'的客户第一页"

**流程：**

1. 确认意图为列表/清单（含上述示例）→ **必须**调用 `get_list`，**禁止**改用 `search`
2. 若用户限定 GTM / 销售 / 状态，先通过对应技能拿 ID；否则相关字段保持 `0` 或 `[]`
2. `customer_name` trim 后空串则不传或传 `""`
3. `page_size` 默认 20，`page` 从 1 开始
4. 调用 `get_list`
5. 错误含「无权限」→ 告知用户无 list 权限
6. `total == 0` 或 `customers` 为空 → 页眉注明「未匹配到客户」，`tbody` 可为空
7. **按 HTML 模板输出**（必须执行）：
   - 读取 `../templates/customer_get_list.html` 的版式与样式
   - 用 `total`、`page`、`customers` 填充页眉、状态徽章与表格行（字段映射见 [get_list_report.md](customer_get_list_report.md)）
   - 写入临时 HTML 文件（如 `/tmp/lfy-customer-get_list-<时间戳>.html`）
   - **用浏览器打开**：macOS 执行 `open "<绝对路径>"`；Linux 执行 `xdg-open "<绝对路径>"`
8. 对话中仅简要说明：报告已在浏览器打开、共 total 条、当前第 page 页、文件路径；勿再贴 Markdown 大表

### 修改客户

**经典 query 示例：**

- "把客户 XX 的状态改成'可能性客户'"
- "给客户 123 改个简称"
- "把这个客户的区域改成华东"

**流程：**

1. 拿到 `customer_id`（已知则直接用；只有名称时先 `search`）
2. **判断要改的字段是否下拉字段**（状态/标签/区域/行业）：
   - 是 → **必须**先 `lfy-cli base get_options '{"object_id": <customer_id>, "property": "customer_status", "cli": true}'` 拿到目标选项的 `id`，再按 [customer_update.md](customer_update.md) 映射表用 `status_id / tags / region_id / industry_id` 写回。**禁止**把中文名称当 `updates` 的键或值。
   - 否（如 `customer_name`、`customer_alias` 等自由文本）→ 直接写值
3. 调用 `update_customer`，`updates` 里**只放要改的键**
4. 返回 `参数错误` → 多半是把名称当成了 id 或用错键名（如误用 `customer_status` 应为 `status_id`），按第 2 步换成 id 后重试一次

**正例（把状态改成「可能性客户」）：**

```bash
# 1) 拿选项 id
lfy-cli base get_options '{"object_id": 189242165298, "property": "customer_status", "cli": true}'
# 假设返回里「可能性客户」的 id 为 5
# 2) 用 status_id 写回
lfy-cli customer update_customer '{"customer_id": 189242165298, "updates": {"status_id": 5}}'
```

参见 [API 详情](customer_update.md)。
