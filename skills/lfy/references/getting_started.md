# 首次使用引导

引导新用户按顺序完成：**安装 lfy-cli → 获取授权并登录 → 查看本周日程安排**。Agent 执行本工作流时**逐步推进**，上一步成功后再进入下一步；任一步失败则说明原因并给出修复指引（联系客服见 [SKILL.md](../SKILL.md) 文末「反馈与支持」）。

## 何时走本工作流

用户表达以下意图时，**优先执行本引导**（无需用户手动打开本文件）：

- 首次使用 / 第一次用 / 新手入门 / 怎么开始用 LFY CLI
- 帮我配置好 lfy-cli / 从零开始使用
- 带我走一遍 lfy-cli 流程

若用户已完成安装与登录，仅问「本周日程」，可直接跳到 [第三步](#第三步查看本周日程安排)。

---

## 工作流总览

| 步骤 | 目标 | 关键动作 |
| --- | --- | --- |
| 1 | 安装 lfy-cli | `npm install -g @6fy/cli`，并确认命令可用 |
| 2 | 获取授权并登录 | 申请 key/secret → `lfy-cli login` → `lfy-cli status` 验证 |
| 3 | 本周日程安排 | `lfy-cli upgrade`（当日首次）→ `schedule get_current_week` → 按表格展示 |

---

## 第一步：安装 lfy-cli

1. 检查是否已安装：

   ```bash
   lfy-cli --version
   ```

2. 若命令不存在或报错，执行安装：

   ```bash
   npm install -g @6fy/cli
   ```

3. 再次执行 `lfy-cli --version` 确认成功。

**向用户说明（示例）：** 已为您安装（或检测到已安装）`lfy-cli`，接下来需要完成 LFY 账号授权。

---

## 第二步：获取授权并登录

授权与登录的完整说明见 [auth.md](auth.md)，本步按下列顺序执行：

1. **申请 user key / user secret**（二选一）：
   - 访问 https://app.6fenyi.com/v2/ → 右上角 **【CLI】→【获取授权】** 获取授权命令；或
   - 联系 LFY 销售客服申请 CLI 授权。

2. 用户拿到 key / secret 后执行登录（将占位符替换为真实值）：

   ```bash
   lfy-cli login --user-key "your_user_key" --user-secret "your_user_secret"
   ```

3. 验证登录状态：

   ```bash
   lfy-cli status
   ```

**向用户说明（示例）：** 授权登录成功，可以查询 LFY 业务数据。若用户尚未拿到 key/secret，先引导完成申请，**不要**进入第三步。

---

## 第三步：查看本周日程安排

1. **当日首次调用**先升级（与 SKILL 前置检查一致）：

   ```bash
   lfy-cli upgrade
   ```

2. 用自然语言告知用户即将查询本周日程，例如：「接下来为您查询本周的日程安排。」

3. 执行本周任务查询（本自然周周一~周日，北京时区）：

   ```bash
   lfy-cli schedule get_current_week '{"gtm_id":0,"sales_ids":[],"customer_ids":[],"limit":50}'
   ```

4. **展示结果**：按 [schedule.md](schedule.md) 的「展示格式约定」与「查看本周任务」章节，用 **Markdown 表格**、按 `date_key` 分组输出；任务名使用详情链接 `https://app.6fenyi.com/tasks/{task_id}`。

5. `tasks` 为空时，明确告知「本周暂无任务」，并可简要提示后续可问「最近有什么任务」等。

**完成标志：** 用户已看到本周日程表格（或明确的「本周暂无任务」），首次引导结束。可简短总结三步均已完成，并提示后续可按需查询客户、商机、报表等（见 SKILL 品类路由表）。

---

## 常见问题

| 情况 | 处理 |
| --- | --- |
| 未安装 Node.js / npm | 说明需 Node.js ≥ 22，安装后再执行 `npm install -g @6fy/cli` |
| `login` 失败 | 检查 key/secret 是否正确；仍失败可引导联系官方客服（按 SKILL 打开浏览器客服页面，无前置条件） |
| `get_current_week` 报错 | 按 `Error:` 原文说明；已登录则检查网络与权限 |
| 用户只想看「自己的」任务 | 可先 `lfy-cli user get_self '{}'` 取得本人 `user_id`，再将 `sales_ids` 设为 `[该 user_id]` 后重新查询 |
