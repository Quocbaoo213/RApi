use std::collections::HashMap;
use std::time::Duration;

use crate::core::contract::response::ApiResult;
use super::device_preset::{URL_ANSWER_QUESTION, URL_CHECK_SESSION, URL_GET_QUESTION, ZMVC_URL};
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn check_session(&self, session: &str, zcid: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(30));
        let mut params = HashMap::new();
        params.insert("session".to_string(), serde_json::Value::String(session.to_string()));
        self.ctx.post_encrypted_http(URL_CHECK_SESSION, &params, zcid, None, None)
    }

    pub fn get_question(&self, session: &str, zcid: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(30));
        let mut params = HashMap::new();
        params.insert("session".to_string(), serde_json::Value::String(session.to_string()));
        self.ctx.post_encrypted_http(URL_GET_QUESTION, &params, zcid, None, None)
    }

    pub fn answer_question(
        &self,
        session: &str,
        answers: &[&str],
        zcid: &str,
        timeout: Option<Duration>,
    ) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(30));
        let mut params = HashMap::new();
        params.insert("session".to_string(), serde_json::Value::String(session.to_string()));
        let ans_val: Vec<serde_json::Value> = answers.iter().map(|&s| serde_json::Value::String(s.to_string())).collect();
        params.insert("answers".to_string(), serde_json::Value::Array(ans_val));
        self.ctx.post_encrypted_http(URL_ANSWER_QUESTION, &params, zcid, None, None)
    }

    pub fn get_2fa_methods(&self, session: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(30));
        let url = format!("{}/api/v1/2fa/methods?session={}", ZMVC_URL, session);
        let headers = HashMap::new();
        self.ctx.get_json(&url, &headers)
    }

    pub fn verify_2fa_otp(
        &self,
        session: &str,
        method: i32,
        otp: &str,
        zcid: &str,
        timeout: Option<Duration>,
    ) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(30));
        let url = format!("{}/api/v1/2fa/verify?session={}&method={}", ZMVC_URL, session, method);
        let mut params = HashMap::new();
        params.insert("method".to_string(), serde_json::Value::from(method));
        params.insert("code".to_string(), serde_json::Value::String(otp.to_string()));
        self.ctx.post_encrypted_http(&url, &params, zcid, None, None)
    }
}
