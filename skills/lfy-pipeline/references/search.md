# search — 搜索商机

## 接口信息

**命令：**
```bash
lfy-cli pipeline search '{"keywords": "<keywords>"}'
```

**参数：**

| 参数名     | 类型   | 必填 | 说明                   |
| ---------- | ------ | ---- | ---------------------- |
| `keywords` | string | 是   | 搜索关键字，支持模糊匹配 |

## 返回数据

```json
[
  {
    "pipeline_id": 1234,
    "pipeline_name": "商机名称"
  }
]
```

| 字段            | 类型    | 说明       |
| --------------- | ------- | ---------- |
| `pipeline_id`   | integer | 商机唯一 ID |
| `pipeline_name` | string  | 商机名称    |

## 错误处理

- `keywords` 为空：返回错误或不完整结果
- 无匹配结果：返回空数组 `[]`
- 接口异常：返回错误信息
