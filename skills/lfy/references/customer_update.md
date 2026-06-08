# update_customer

## 命令

```bash
lfy-cli customer update_customer '{"customer_id": 123, "updates": {}}'
```

## 权限

需要 per_user 客户模块 **detail** 场景有效权限，且目标客户的 `sales_id` 须在 detail 的 `sales_ids` 白名单内（与「查看客户详情」门禁一致）。

## 参数

| 字段 | 类型 | 说明 |
|------|------|------|
| `customer_id` | int | 必填，>0 |
| `updates` | object | 必填；**仅出现的键会更新**；未知键报错；**空对象 `{}`**：不写库、不刷新 ES，成功返回当前 `updated_time` |

## 下拉字段：名称 → updates 键（硬规则）

**禁止**把中文名称或 `base get_options` 的 `property` 名直接当成 `updates` 的键或值。下拉字段（状态、标签、区域、行业）必须先用 `lfy-cli base get_options` 拿到选项 `id`，再用下表对应的键写回（值为 int id）：

| 用户/详情里的名称 | base get_options property | updates 里要用的键 | 值 |
|------------------|---------------------------|-------------------|----|
| 客户状态 customer_status | customer_status | `status_id` | 选项 id（int） |
| 客户标签 customer_tags | customer_tags | `tags` | 逗号分隔的 taxonomy id |
| 区域 customer_region | customer_region | `region_id` | 选项 id（int），0=清空 |
| 行业 customer_industry | customer_industry | `industry_id` | 选项 id（int），0=清空 |

> 反例（会报 `参数错误`）：`{"updates": {"customer_status": "可能性客户"}}`（用了名称当键和值）
> 正例：先 `base get_options` 查到「可能性客户」id=5 → `{"updates": {"status_id": 5}}`

### `updates` 支持的键（均可选组合）

| 键 | 类型 | 规则 |
|----|------|------|
| `status_id` | int | >0；选项 id **必须**先 `lfy-cli base get_options`（`property=customer_status`，需 `object_id`）取得，禁止直传名称 |
| `customer_name` | string | trim 后非空，≤50 字符（Unicode）；企业内其它激活客户不可重名（自身除外） |
| `annual_procurement_amount` | number | ≥0，最多两位小数 |
| `tags` | string | 逗号分隔标签 **taxonomy id**，空格可忽略；**空字符串表示清空标签**；选项见 `property=customer_tags` |
| `avg_collection_days` | int | 0～9999 |
| `region_id` | int | ≥0；**0** 表示无区域（清空）；选项见 `property=customer_region` |
| `industry_id` | int | ≥0；**0** 表示无行业（清空）；选项见 `property=customer_industry` |
| `customer_alias` | string | 允许空字符串 |

## 成功响应示例（MCP 层带 code）

```json
{
  "code": 200,
  "customer_id": 87357490413632,
  "updated_time": "2026-05-08 10:00:00"
}
```

`updated_time` 为北京时间 `YYYY-MM-DD HH:mm:ss`。

## 常见错误（CLI 为 `Error: ...`）

| 场景 | 文案 |
|------|------|
| 模块无权限或销售范围无权限 | `您暂无权限` |
| 客户不存在或未激活 | `客户不存在` |
| 名称与其它客户冲突 | `客户名称已存在` |
| JSON/字段非法 | `参数错误` |
| 其它服务端错误 | `更新失败，请稍后重试` |
