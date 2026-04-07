use crate::{config, json_rpc};
use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static TOOLS_CACHE: RefCell<HashMap<String, Vec<serde_json::Value>>> = RefCell::new(HashMap::new());
}

/// 通过 JSON-RPC 拉取并打印某品类下可用工具列表。
pub async fn show_category_tools(category: &str) -> Result<()> {
    let res = json_rpc::send(category, "tools/list", None, None).await?;

    let Some(tools) = res
        .get("result")
        .and_then(|r| r.get("tools"))
        .and_then(|r| r.as_array())
    else {
        anyhow::bail!("无法获取 {} 品类的工具列表: {res}", category);
    };

    TOOLS_CACHE.with(|cache| {
        let mut cache_ref = cache.borrow_mut();
        cache_ref.insert(category.to_string(), tools.clone());
    });

    let category_description = config::get_categories()
        .iter()
        .find(|info| info.name == category)
        .map(|info| info.description)
        .unwrap_or("");

    let bin = env!("CARGO_BIN_NAME");

    println!("# {} {}", category, category_description);
    println!();
    println!("使用方式:");
    println!("    {} {} <method> [json_args]", bin, category);
    println!();
    println!("选项:");
    println!("    -h, --help        显示详细的工具 schema 信息");
    println!();
    for tool in tools {
        let Some(name) = tool.get("name").and_then(|n| n.as_str()) else {
            continue;
        };
        println!();
        println!("## {}", name);
        if let Some(description) = tool.get("description").and_then(|d| d.as_str()) {
            println!();
            println!("{}", description);
        }
    }

    Ok(())
}

pub async fn show_tool_help(category: &str, tool_name: &str) -> Result<()> {
    let tools = TOOLS_CACHE.with(|cache| {
        let cache_ref = cache.borrow();
        cache_ref.get(category).cloned()
    });

    let tools = if let Some(tools) = tools {
        tools
    } else {
        let res = json_rpc::send(category, "tools/list", None, None).await?;

        let tools = res
            .get("result")
            .and_then(|r| r.get("tools"))
            .and_then(|r| r.as_array())
            .cloned()
            .unwrap_or_default();

        TOOLS_CACHE.with(|cache| {
            let mut cache_ref = cache.borrow_mut();
            cache_ref.insert(category.to_string(), tools.clone());
        });

        tools
    };

    let tool = tools.iter().find(|t| {
        t.get("name")
            .and_then(|n| n.as_str())
            .map(|name| name == tool_name)
            .unwrap_or(false)
    });

    if let Some(tool) = tool {
        println!("# {} - {}", category, tool_name);
        println!();

        if let Some(description) = tool.get("description").and_then(|d| d.as_str()) {
            println!("## 描述");
            println!("{}", description);
            println!();
        }

        if let Some(input_schema) = tool.get("inputSchema") {
            println!("## 输入参数");
            println!("```json");
            println!(
                "{}",
                serde_json::to_string_pretty(input_schema).unwrap_or_default()
            );
            println!("```");
            println!();
        }

        if let Some(parameters) = tool.get("parameters") {
            println!("## 参数");
            println!("```json");
            println!(
                "{}",
                serde_json::to_string_pretty(parameters).unwrap_or_default()
            );
            println!("```");
        }
    } else {
        println!("未找到工具: {} {}", category, tool_name);
    }

    Ok(())
}
