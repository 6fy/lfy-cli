---
name: lfy-pipeline
description: 商机查询技能。适用于通过关键字搜索商机列表。当用户需要按关键字搜索商机时使用此技能。
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli pipeline --help"
---

# 商机查询技能

> `lfy-cli` 是陆份仪提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli pipeline <接口名> ' '` 与商机系统交互。

## 注意事项

- `keywords` 为空时可能返回错误或不完整结果
- 若 `errcode` 不为 `0` 或返回格式异常，需告知用户错误信息
- 若搜索结果为空，告知用户未找到对应商机

---

## 接口列表

### search — 搜索商机

```bash
lfy-cli pipeline search '{"keywords": "<keywords>"}'
```

按关键字搜索商机，支持模糊匹配。

参见 [API 详情](references/search.md)。

---

## 返回格式

### search 返回格式

```json
[
  {
    "pipeline_id": <pipeline_id>,
    "pipeline_name": "<pipeline_name>"
  }
]
```

| 字段            | 类型    | 说明       |
| --------------- | ------- | ---------- |
| `pipeline_id`   | integer | 商机唯一 ID |
| `pipeline_name` | string  | 商机名称    |

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
5. 若找到多个匹配，展示候选列表请用户确认

**展示结果：**

找到商机时：
```
📇 找到商机：
- 商机名称：<pipeline_name>
- 商机ID：<pipeline_id>
```

多个匹配时：
```
🔍 找到多个匹配商机，请确认您要查询的是哪个：

1. <pipeline_name_1>（ID：<pipeline_id_1>）
2. <pipeline_name_2>（ID：<pipeline_id_2>）

请问您要查询的是哪一个？
```

未找到时：
```
未找到包含"<keywords>"的商机，请尝试其他关键字。
```
