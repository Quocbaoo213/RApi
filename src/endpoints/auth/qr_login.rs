use std::collections::HashMap;
use std::time::Duration;

use crate::core::contract::response::ApiResult;
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn request_qr(&self, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(10));
        let headers = HashMap::new();
        let url = "https://id.zalo.me/account/get-qr";
        self.ctx.get_json(url, &headers)
    }

    pub fn check_qr_status(&self, qr_code: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(10));
        let mut headers = HashMap::new();
        headers.insert("qr_code".to_string(), qr_code.to_string());
        let url = format!("https://id.zalo.me/account/check-qr?code={}", qr_code);
        self.ctx.get_json(&url, &headers)
    }
}
