use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::IdentityEndpoint;

impl IdentityEndpoint {
    pub fn get_avatar(&self, user_id: i64, high_res: &str, prefer_jxl: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("getAvatar".to_string()));
        payload.insert("userId".to_string(), serde_json::Value::from(user_id));
        payload.insert("high_res".to_string(), serde_json::Value::String(high_res.to_string()));
        payload.insert("prefer_jxl".to_string(), serde_json::Value::String(prefer_jxl.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(154, 0, 1, 1, &enc, true, to)
    }
}
