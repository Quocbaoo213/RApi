use std::collections::HashMap;
use std::time::Duration;

use crate::core::contract::response::ApiResult;
use crate::core::security::aes::password_hash;
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn authenticate(
        &self,
        phone: &str,
        password: &str,
        zcid: &str,
        timeout: Option<Duration>,
    ) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(10));
        let enc_pwd = password_hash(phone, password, None)?;
        let mut params = HashMap::new();
        params.insert("phone".to_string(), serde_json::Value::String(phone.to_string()));
        params.insert("password".to_string(), serde_json::Value::String(enc_pwd));
        params.insert("zcid".to_string(), serde_json::Value::String(zcid.to_string()));
        let url = "https://id.zalo.me/account/login";
        self.ctx.post_encrypted_http(url, &params, zcid, None, None)
    }
}
