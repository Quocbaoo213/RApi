use std::collections::HashMap;
use std::time::Duration;

use crate::core::contract::response::ApiResult;
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn submit_captcha(
        &self,
        captcha_token: &str,
        solution: &str,
        zcid: &str,
        timeout: Option<Duration>,
    ) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(10));
        let mut params = HashMap::new();
        params.insert("token".to_string(), serde_json::Value::String(captcha_token.to_string()));
        params.insert("solution".to_string(), serde_json::Value::String(solution.to_string()));
        let url = "https://id.zalo.me/account/captcha/submit";
        self.ctx.post_encrypted_http(url, &params, zcid, None, None)
    }
}
