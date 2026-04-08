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
        description: "客户 - 客户增删改及可用性查询",
    });

    categories.push(CategoryInfo {
        name: "pipeline",
        description: "Pipeline - 商机管道相关查询",
    });

    categories.push(CategoryInfo {
        name: "user",
        description: "用户 - 获取本人用户信息",
    });

    categories.push(CategoryInfo {
        name: "ops",
        description: "Ops - 企业运营相关查询",
    });

    categories
}
