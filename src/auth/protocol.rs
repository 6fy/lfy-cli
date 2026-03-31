use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredentials {
    pub user_key: String,
    pub user_secret: String,
    pub create_time: u64,
}

impl UserCredentials {
    pub fn new(user_key: String, user_secret: String) -> Self {
        let create_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            user_key,
            user_secret,
            create_time,
        }
    }
}
