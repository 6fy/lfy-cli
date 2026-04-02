# LFY CLI Server URL 配置设计

## 背景

支持通过 `lfy-cli init --server-url <url>` 命令设置 MCP Server 地址，并持久化保存到本地配置文件。

后续所有 MCP 请求优先使用保存的地址，简化环境变量配置。

## 配置优先级

1. 环境变量 `LFY_MCP_CONFIG_ENDPOINT`（最高优先）
2. 配置文件 `~/.config/lfy/settings.json`
3. 硬编码默认值 `http://localhost:16000`

## 实现方案

### 1. 配置文件

路径：`~/.config/lfy/settings.json`

```json
{
  "server_url": "http://127.0.0.1:16000"
}
```

### 2. 修改 constants.rs

新增 `settings.rs` 模块或直接在 `constants.rs` 中添加：

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub server_url: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self { server_url: None }
    }
}

pub fn settings_path() -> std::path::PathBuf {
    config_dir().join("settings.json")
}

pub fn load_settings() -> Settings {
    let path = settings_path();
    if let Ok(data) = std::fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        Settings::default()
    }
}

pub fn save_settings(settings: &Settings) -> Result<()> {
    let path = settings_path();
    let data = serde_json::to_string_pretty(settings)?;
    std::fs::write(path, data)?;
    Ok(())
}

fn load_server_url_from_config() -> Option<String> {
    load_settings().server_url.filter(|s| !s.is_empty())
}

pub fn mcp_config_endpoint() -> String {
    // 1. 环境变量（最高优先）
    if let Ok(url) = std::env::var(env::MCP_CONFIG_ENDPOINT) {
        if !url.is_empty() {
            return url;
        }
    }

    // 2. 配置文件
    if let Some(url) = load_server_url_from_config() {
        return url;
    }

    // 3. 默认值
    DEFAULT_MCP_CONFIG_ENDPOINT.to_string()
}
```

### 3. 修改 init.rs

```rust
#[derive(Args)]
pub struct InitArgs {
    #[arg(long, help = "仅刷新 MCP 配置缓存（不更新 User Key / User Secret）")]
    refresh: bool,

    #[arg(long, help = "User Key（非交互时可传入）")]
    user_key: Option<String>,

    #[arg(long, help = "User Secret（非交互时可传入，注意 shell 历史）")]
    user_secret: Option<String>,

    #[arg(long, help = "MCP Server 地址")]
    server_url: Option<String>,
}

pub async fn handle_init_cmd(matches: &ArgMatches) -> Result<()> {
    let args = InitArgs::from_arg_matches(matches)?;

    // 处理 server_url 配置
    if let Some(url) = args.server_url {
        let mut settings = load_settings();
        settings.server_url = Some(url.clone());
        save_settings(&settings)?;
        println!("Server 地址已保存: {}", url);
    }

    // ... 后续逻辑不变 ...
}
```

### 4. 更新 README.md

在"变更服务器地址"部分添加说明：

```markdown
# 变更服务器地址

## 方式一：命令行参数（推荐）
lfy-cli init --server-url http://127.0.0.1:16000

## 方式二：环境变量
LFY_MCP_CONFIG_ENDPOINT=http://127.0.0.1:16000 lfy-cli init

# 优先级：环境变量 > 命令行参数 > 默认值
```

## 注意事项

- 空字符串视为"未设置"，会继续尝试后续优先级
- `settings.json` 不存在时会自动创建
- 仅 `init` 命令支持 `--server-url`，其他命令通过环境变量覆盖
