use std::collections::HashMap;
use std::time::Duration;

use crate::core::contract::response::ApiResult;
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn verify_2fa(
        &self,
        token: &str,
        code: &str,
        zcid: &str,
        timeout: Option<Duration>,
    ) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(10));
        let mut params = HashMap::new();
        params.insert("token".to_string(), serde_json::Value::String(token.to_string()));
        params.insert("code".to_string(), serde_json::Value::String(code.to_string()));
        let url = "https://id.zalo.me/account/2fa/verify";
        self.ctx.post_encrypted_http(url, &params, zcid, None, None)
    }
}
