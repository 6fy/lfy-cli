use crate::{auth, config, help, json_rpc};

use anyhow::Result;
use clap::{ArgMatches, Args, FromArgMatches};
use serde_json::json;

#[derive(Args)]
pub struct CallArgs {
    /// 要调用的工具方法名
    #[arg(value_name = "method")]
    pub method: Option<String>,

    /// JSON 格式的参数
    #[arg(value_name = "args")]
    pub args: Option<String>,

    #[arg(long, short)]
    pub help: bool,
}

pub async fn handle_call_cmd(category_name: &str, matches: &ArgMatches) -> Result<()> {
    let args = CallArgs::from_arg_matches(matches)?;

    let categories = config::get_categories();
    if !categories.iter().any(|c| c.name == category_name) {
        anyhow::bail!("无效命令：{}", category_name);
    }

    if args.help {
        if let Some(method) = args.method.as_deref() {
            let full_method = if method.contains('/') {
                method.to_string()
            } else {
                format!("{}/{}", category_name, method)
            };
            help::show_tool_help(category_name, &full_method).await?;
        } else {
            help::show_category_tools(category_name).await?;
        }
        return Ok(());
    }

    let Some(method) = args.method.as_deref() else {
        help::show_category_tools(category_name).await?;
        return Ok(());
    };

    let args = args.args.as_deref();

    if args.is_none() {
        help::show_tool_help(category_name, method).await?;
        return Ok(());
    }

    let parsed_args = if let Some(args) = args {
        serde_json::from_str(args)?
    } else {
        json!({})
    };

    let full_method = if method.contains('/') {
        method.to_string()
    } else {
        format!("{}/{}", category_name, method)
    };

    // Inject auth for customer/* tools (except customer/is_available).
    // The server expects credentials under `arguments.auth`.
    let mut parsed_args = parsed_args;
    if category_name == "customer" && full_method != "customer/is_available" {
        let obj = parsed_args.as_object_mut().ok_or_else(|| {
            anyhow::anyhow!("参数必须是 JSON 对象，以便注入 auth")
        })?;

        if !obj.contains_key("auth") {
            let creds = auth::get_credentials()
                .ok_or_else(|| anyhow::anyhow!("未找到凭证，请先运行 `lfy-cli init`"))?;

            obj.insert(
                "auth".to_string(),
                json!({
                    "user_key": creds.user_key,
                    "user_secret": creds.user_secret
                }),
            );
        }
    }

    let params = json!({
        "name": full_method,
        "arguments": parsed_args,
    });

    let res = json_rpc::send(category_name, "tools/call", Some(params), None).await?;

    if let Some(result) = res.get("result") {
        println!("{}", result);
    }

    Ok(())
}
