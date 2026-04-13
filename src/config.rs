#[derive(Debug, Clone)]
pub struct CategoryInfo {
    pub name: &'static str,
    pub description: &'static str,
}

/// 支持的 CLI 品类（与 MCP `biz_type` 对应）。按字母序追加。
pub fn get_categories() -> Vec<CategoryInfo> {
    let mut categories = vec![];

    categories.push(CategoryInfo {
        name: "customer",
        description: "客户 - 提供销售核心场景部分可用性查询",
    });

    categories.push(CategoryInfo {
        name: "ops",
        description: "企基 - 提供企业运营基础数据查询",
    });

    categories.push(CategoryInfo {
        name: "pipeline",
        description: "商机 - 提供销售核心场景部分可用性查询",
    });

    categories.push(CategoryInfo {
        name: "report",
        description: "报表 - 销售目标等指标查询",
    });

    categories.push(CategoryInfo {
        name: "schedule",
        description: "日程 - 提供日历中工作任务相关查询",
    });

    categories.push(CategoryInfo {
        name: "user",
        description: "用户 - 提供用户信息/销售人员查询",
    });

    categories
}
