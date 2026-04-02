use serde::{Deserialize, Serialize};

use crate::constants::config_dir;

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

pub fn save_settings(settings: &Settings) -> anyhow::Result<()> {
    use std::io::Write;
    let path = settings_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = std::fs::File::create(&path)?;
    write!(file, "{}", serde_json::to_string_pretty(settings)?)?;
    Ok(())
}
