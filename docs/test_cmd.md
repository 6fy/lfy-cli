## 1.6 修改客户信息

```bash
#修改别名
cargo run -- customer update_customer '{"customer_id": 87357490413632, "updates": {"customer_alias": "新别名"}}'

```

## 1.7 客户列表

```bash
cargo run -- customer get_list '{"gtm_id":0,"customer_name":"","customer_status_ids":[],"sales_ids":[],"page_size":20,"page":1}'
```

## 1.8 联系人列表

```bash
# 默认分页（GTM 不过滤、姓名不过滤、负责人用 list 权限白名单）
cargo run -- contact get_list '{"gtm_id":0,"contacts_name":"","sales_ids":[],"page_size":20,"page":1}'

# 按 GTM 筛选
cargo run -- contact get_list '{"gtm_id":24685820686,"contacts_name":"","sales_ids":[],"page_size":20,"page":1}'

# 按姓名模糊搜索
cargo run -- contact get_list '{"gtm_id":0,"contacts_name":"张","sales_ids":[],"page_size":10,"page":1}'
```

## 6.4 创建日程

```bash
cargo run -- schedule create_task '{"task_name":"名称","start_time":"2026-06-04","end_time":"2026-06-07"}'

cargo run -- schedule create_task '{"task_name":"名称","end_time":"2026-06-12"}'
```

## 6.5 获取日程列表

```bash
cargo run -- schedule get_recent_tasks '{}'
```

## 10.2 测试获取下拉数据命令

```bash
cargo run -- base get_options '{"object_id": 87357490413632, "property": "customer_status", "cli": true}'

cargo run -- base get_options '{"object_id": 87357490413632, "property": "customer_tags", "cli": true}'

cargo run -- base get_options '{"object_id": 87357490413632, "property": "customer_region", "cli": true}'

cargo run -- base get_options '{"object_id": 87357490413632, "property": "customer_industry", "cli": true}'

cargo run -- pipeline get_list '{"gtm_id":0,"pipeline_name":"","pipeline_status_ids":[],"sales_ids":[],"page_size":20,"page":1}'

```