use std::collections::HashMap;
use std::time::Duration;

use crate::core::contract::response::ApiResult;
use crate::core::security::aes::password_hash;
use super::device_preset::{URL_ACTIVE_BY_PASSWORD, URL_PHONE_VERIFY};
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn verify_phone(
        &self,
        phone: &str,
        zcid: &str,
        timeout: Option<Duration>,
    ) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(30));
        let mut params = HashMap::new();
        params.insert("phone".to_string(), serde_json::Value::String(phone.to_string()));
        params.insert("zcid".to_string(), serde_json::Value::String(format!("1_{}", zcid)));
        self.ctx.post_encrypted_http(URL_PHONE_VERIFY, &params, zcid, None, None)
    }

    pub fn authenticate(
        &self,
        phone: &str,
        password: &str,
        zcid: &str,
        session_token: Option<&str>,
        timeout: Option<Duration>,
    ) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(30));
        let enc_pwd = password_hash(phone, password, None)?;
        let mut params = HashMap::new();
        params.insert("phone".to_string(), serde_json::Value::String(phone.to_string()));
        params.insert("password".to_string(), serde_json::Value::String(enc_pwd));
        params.insert("zcid".to_string(), serde_json::Value::String(format!("1_{}", zcid)));
        if let Some(st) = session_token {
            params.insert("session_token".to_string(), serde_json::Value::String(st.to_string()));
        }
        self.ctx.post_encrypted_http(URL_ACTIVE_BY_PASSWORD, &params, zcid, None, None)
    }
}
