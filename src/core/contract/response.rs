use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResult {
    pub error_code: i32,
    pub error_message: String,
    pub data: Option<HashMap<String, serde_json::Value>>,
    pub raw_params: Option<Vec<u8>>,
    pub success: bool,
}

impl ApiResult {
    pub fn success(data: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self {
            error_code: 0,
            error_message: "successful".to_string(),
            data,
            raw_params: None,
            success: true,
        }
    }

    pub fn error(code: i32, msg: impl Into<String>) -> Self {
        Self {
            error_code: code,
            error_message: msg.into(),
            data: None,
            raw_params: None,
            success: false,
        }
    }
}
