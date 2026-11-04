use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionProfile {
    #[serde(default)]
    pub uid: i64,
    #[serde(default)]
    pub session_key: String,
    #[serde(default)]
    pub dk_hex: String,
    #[serde(default)]
    pub ksid: String,
    #[serde(default)]
    pub cryptkey: String,
    #[serde(default)]
    pub ss_pub_key: String,
    #[serde(default)]
    pub socket_servers: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub zcid: String,
}

impl SessionProfile {
    pub fn load_from_file(file_path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), String> {
        let content = serde_json::to_string_pretty(self).map_err(|e| e.to_string())?;
        fs::write(file_path, content).map_err(|e| e.to_string())
    }
}
