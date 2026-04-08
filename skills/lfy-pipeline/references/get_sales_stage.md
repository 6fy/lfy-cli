# get_sales_stage — 获取商机阶段

## 接口信息

**命令：**
```bash
lfy-cli pipeline get_sales_stage '{"gtm_id": <gtm_id>}'
```

**参数：**

| 参数名   | 类型    | 必填 | 说明       |
| -------- | ------- | ---- | ---------- |
| `gtm_id` | integer | 是   | GTM 业务 ID |

## 返回数据

```json
[
  {
    "stage_id": 123,
    "stage_name": "0% 线索阶段",
    "milestone_goal": "里程碑目标",
    "value_proposition": "价值主张",
    "suggested_stage_days": 14
  }
]
```

| 字段                   | 类型    | 说明             |
| ---------------------- | ------- | ---------------- |
| `stage_id`             | integer | 阶段唯一 ID      |
| `stage_name`           | string  | 阶段名称         |
| `milestone_goal`       | string  | 里程碑目标       |
| `value_proposition`    | string  | 价值主张         |
| `suggested_stage_days` | integer | 建议阶段天数     |

## 错误处理

- `gtm_id` 为空或格式错误：返回错误或不完整结果
- 无匹配结果：返回空数组 `[]`
- 接口异常：返回错误信息
