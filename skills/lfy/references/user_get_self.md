# get_self — 获取本人用户信息

## 接口说明

获取当前登录用户的基本信息，包括用户 ID、姓名及所属组织名称。

## 请求示例

```bash
lfy-cli user get_self '{}'
```

## 返回示例

```json
{
  "user_id": 123,
  "user_name": "张三",
  "org_name": "轻舟公司"
}
```

## 字段说明

| 字段       | 类型    | 说明           |
| ---------- | ------- | -------------- |
| `user_id`  | integer | 用户唯一 ID    |
| `user_name` | string  | 用户姓名       |
| `org_name` | string  | 所属组织名称   |
