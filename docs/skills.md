## Skills 导航

仓库内置一个统一的 Agent Skill —— **LFY**，位于 `skills/lfy/` 目录下。`SKILL.md` 为轻路由主入口（品类路由表 + 工作流清单 + 通用约定 + 跨品类协作）；每个品类的接口清单与详细工作流位于 `references/<品类>.md`，按需加载（渐进式披露）。

## Agent Skill：LFY

| 项 | 说明 |
|----|------|
| 技能名 | `LFY` |
| 入口 | `skills/lfy/SKILL.md` |
| 调用格式 | `lfy-cli <品类> <方法名> '<json入参>'` |

覆盖的 8 个品类（指南均位于 `skills/lfy/references/<品类>.md`）：

| 品类 | 指南 | 说明 |
|------|------|------|
| `customer` | `references/customer.md` | 客户搜索、我的客户清单、详情、GTM、创建/修改 |
| `pipeline` | `references/pipeline.md` | 商机搜索、阶段、详情、待签单、列表、创建/修改 |
| `report` | `references/report.md` | 销售目标、销售大局观、GTM 财务报表（只读） |
| `user` | `references/user.md` | 本人信息、销售人员名单（只读） |
| `ops` | `references/ops.md` | 财年、当前周（只读） |
| `schedule` | `references/schedule.md` | 最近任务、本周任务（只读） |
| `contact` | `references/contact.md` | 联系人列表（只读） |
| `base` | `references/base.md` | 客户/商机编辑场景下拉选项（只读） |
