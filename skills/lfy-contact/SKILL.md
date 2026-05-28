---
name: lfy-contact
description: 联系人技能。适用于分页查询联系人列表，按姓名模糊搜索、按负责人筛选。当用户需要查看联系人列表、检索联系人时使用此技能。
version: 1.0.0
metadata:
  requires:
    bins: ["lfy-cli"]
  cliHelp: "lfy-cli contact --help"
---

# 联系人技能

> `lfy-cli` 是 LFY 提供的命令行程序，所有操作通过执行 `lfy-cli` 命令完成。

通过 `lfy-cli contact <方法名> '<json>'` 与联系人数据交互。

## 注意事项

- 需具备联系人模块 **list** 权限（`per_user` `category_id=1544862211`）
- `sales_ids` 为空表示使用当前用户 list 权限白名单；非空时与白名单求交集
- 若返回错误，按 `Error: {message}` 告知用户

## 接口列表

### 联系人列表 (get_list)

```bash
lfy-cli contact get_list '{"contacts_name":"","sales_ids":[],"page_size":20,"page":1}'
```

分页查询当前用户 list 权限范围内的联系人，支持按姓名（ILIKE 不区分大小写）、负责人过滤。lfy-cli-server 已剥 `data`，CLI 直接见 `{name, total, contacts}`。

参见 [API 详情](references/get_list.md)。

## 典型工作流

### 查询联系人列表

1. 确认用户要查的是「联系人列表」而非客户/商机列表
2. 执行 `contact get_list`，按需设置 `contacts_name`、`sales_ids`、`page`、`page_size`
3. 展示 `total` 与当前页 `contacts`：姓名、负责人、电话/邮箱数组、所属客户、相关机会、职位、态度、创建日期

成功时：说明总条数，列出联系人姓名、负责人、客户名、机会名；`phone`/`email` 为空数组时说明「无」。
