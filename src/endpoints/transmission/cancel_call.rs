use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn cancel_call(&self, user_id: i64, call_id: i64, reason: &str, is_video: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("cancelCall".to_string()));
        payload.insert("userId".to_string(), serde_json::Value::from(user_id));
        payload.insert("callId".to_string(), serde_json::Value::from(call_id));
        payload.insert("reason".to_string(), serde_json::Value::String(reason.to_string()));
        payload.insert("isVideo".to_string(), serde_json::Value::from(is_video));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(406, 0, 1, 1, &enc, true, to)
    }
}
