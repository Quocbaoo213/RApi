use std::collections::HashMap;
use std::time::Duration;

use crate::core::contract::response::ApiResult;
use crate::core::security::aes::{compute_sig, decrypt_http_data, encrypt_http_data};
use crate::core::security::key::DEFAULT_SECRET;

pub struct HttpClient {
    agent: ureq::Agent,
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpClient {
    pub fn new() -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(15))
            .timeout_write(Duration::from_secs(15))
            .build();
        Self { agent }
    }

    pub fn post_encrypted(
        &self,
        url: &str,
        params: &HashMap<String, serde_json::Value>,
        zcid: &str,
        api_key: Option<&str>,
        secret: Option<&str>,
    ) -> Result<ApiResult, String> {
        let key = api_key.unwrap_or("3423724394324");
        let sec = secret.unwrap_or(DEFAULT_SECRET);
        let data_hex = encrypt_http_data(params, zcid)?;
        let sig = compute_sig(&data_hex, key, Some(sec));

        let mut req = self
            .agent
            .post(url)
            .set("Content-Type", "application/x-www-form-urlencoded")
            .set("User-Agent", "Mozilla/5.0");

        if !zcid.is_empty() {
            req = req.set("zcid", zcid);
        }

        let body = format!("sig={}&api_key={}&data={}", sig, key, data_hex);
        let resp = req.send_string(&body).map_err(|e| e.to_string())?;
        let resp_str = resp.into_string().map_err(|e| e.to_string())?;

        let res_map: HashMap<String, serde_json::Value> = match serde_json::from_str(&resp_str) {
            Ok(m) => m,
            Err(_) => match decrypt_http_data(&resp_str, zcid) {
                Ok(m) => m,
                Err(_) => {
                    let mut fallback = HashMap::new();
                    fallback.insert("raw".to_string(), serde_json::Value::String(resp_str));
                    return Ok(ApiResult::success(Some(fallback)));
                }
            },
        };

        let mut ec = 0;
        if let Some(v) = res_map.get("error_code").and_then(|v| v.as_i64()) {
            ec = v as i32;
        }
        let em = res_map
            .get("error_message")
            .and_then(|v| v.as_str())
            .unwrap_or("successful")
            .to_string();

        Ok(ApiResult {
            error_code: ec,
            error_message: em,
            data: Some(res_map),
            raw_params: None,
            success: ec == 0,
        })
    }

    pub fn get_json(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<ApiResult, String> {
        let mut req = self.agent.get(url).set("User-Agent", "Mozilla/5.0");
        for (k, v) in headers {
            req = req.set(k, v);
        }
        let resp = req.call().map_err(|e| e.to_string())?;
        let resp_str = resp.into_string().map_err(|e| e.to_string())?;

        let res_map: HashMap<String, serde_json::Value> = match serde_json::from_str(&resp_str) {
            Ok(m) => m,
            Err(_) => {
                let mut fallback = HashMap::new();
                fallback.insert("raw".to_string(), serde_json::Value::String(resp_str));
                return Ok(ApiResult::success(Some(fallback)));
            }
        };

        let mut ec = 0;
        if let Some(v) = res_map.get("error_code").and_then(|v| v.as_i64()) {
            ec = v as i32;
        }
        let em = res_map
            .get("error_message")
            .and_then(|v| v.as_str())
            .unwrap_or("successful")
            .to_string();

        Ok(ApiResult {
            error_code: ec,
            error_message: em,
            data: Some(res_map),
            raw_params: None,
            success: ec == 0,
        })
    }
}
