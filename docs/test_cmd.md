## 1.6 修改客户信息

```bash
#修改别名
cargo run -- customer update_customer '{"customer_id": 87357490413632, "updates": {"customer_alias": "新别名"}}'

```

## 10.2 测试获取下拉数据命令

```bash
cargo run -- base get_options '{"object_id": 87357490413632, "property": "customer_status", "cli": true}'

cargo run -- base get_options '{"object_id": 87357490413632, "property": "customer_tags", "cli": true}'

cargo run -- base get_options '{"object_id": 87357490413632, "property": "customer_region", "cli": true}'

cargo run -- base get_options '{"object_id": 87357490413632, "property": "customer_industry", "cli": true}'
```