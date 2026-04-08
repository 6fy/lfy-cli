# get_gtms — 获取 GTM 列表

获取所有 GTM（Go To Market）业务线分类列表。

## 命令格式

```bash
lfy-cli customer get_gtms '{}'
```

## 参数说明

无需参数，传入空对象即可。

## 返回格式

```json
[
  {
    "gtm_id": 123,
    "gtm_name": "华北区"
  },
  {
    "gtm_id": 124,
    "gtm_name": "华东区"
  },
  {
    "gtm_id": 125,
    "gtm_name": "华南区"
  }
]
```

## 返回字段说明

| 字段      | 类型    | 说明           |
| --------- | ------- | -------------- |
| `gtm_id`  | integer | GTM 业务线 ID  |
| `gtm_name` | string  | GTM 业务线名称 |

## 使用场景

- 用户询问有哪些 GTM 业务线时使用
- 用户需要了解客户的 GTM 分类归属时使用
- 作为其他客户相关功能的辅助信息查询

## 错误处理

- 若 `errcode` 不为 `0`，展示错误信息并终止流程
- 若返回格式异常，告知用户并建议稍后重试
