use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::IdentityEndpoint;

impl IdentityEndpoint {
    pub fn update_privacy(&self, setting: &str, value: &str, extra_str: &HashMap<String, serde_json::Value>, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("updatePrivacy".to_string()));
        payload.insert("setting".to_string(), serde_json::Value::String(setting.to_string()));
        payload.insert("value".to_string(), serde_json::Value::String(value.to_string()));
        payload.insert("extra_str".to_string(), serde_json::to_value(extra_str).unwrap_or_default());
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(273, 1, 1, 1, &enc, true, to)
    }
}
