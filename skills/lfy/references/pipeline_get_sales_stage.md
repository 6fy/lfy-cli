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
    "suggested_stage_days": 14,
    "type_value": 10,
    "type_name": "线索阶段"
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
| `type_value`           | integer | 阶段类型值：10 线索阶段，20 机会阶段，30 交付阶段，40 回款阶段；无类型配置时为 `0` |
| `type_name`            | string  | 阶段类型名称；无类型配置时为 `""` |

## 错误处理

- `gtm_id` 为空或格式错误：返回错误或不完整结果
- 无匹配结果：返回空数组 `[]`
- 接口异常：返回错误信息
