use std::collections::HashMap;
use std::time::Duration;

use crate::core::contract::response::ApiResult;
use super::device_preset::URL_CAPTCHA_VERIFY;
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn submit_captcha(
        &self,
        captcha_session: &str,
        left: i32,
        start_time: i64,
        stop_time: i64,
        zcid: &str,
        timeout: Option<Duration>,
    ) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(30));
        let mut params = HashMap::new();
        params.insert("left".to_string(), serde_json::Value::from(left));
        params.insert("startTime".to_string(), serde_json::Value::from(start_time));
        params.insert("stopTime".to_string(), serde_json::Value::from(stop_time));
        let url = format!("{}?session={}", URL_CAPTCHA_VERIFY, captcha_session);
        self.ctx.post_encrypted_http(&url, &params, zcid, None, None)
    }
}
