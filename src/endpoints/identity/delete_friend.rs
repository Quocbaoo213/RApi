use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::IdentityEndpoint;

impl IdentityEndpoint {
    pub fn delete_friend(&self, user_id: i64, tracking_source: i32, vercode: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("delete_friend".to_string()));
        payload.insert("userId".to_string(), serde_json::Value::from(user_id));
        payload.insert("trackingSource".to_string(), serde_json::Value::from(tracking_source));
        payload.insert("vercode".to_string(), serde_json::Value::from(vercode));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(378, 0, 1, 1, &enc, true, to)
    }
}
