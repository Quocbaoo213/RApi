use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn control_group_call_media(&self, group_id: i64, user_id: i64, call_type: i32, duration: &str, cmsg_id: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("controlGroupCallMedia".to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("userId".to_string(), serde_json::Value::from(user_id));
        payload.insert("callType".to_string(), serde_json::Value::from(call_type));
        payload.insert("userId".to_string(), serde_json::Value::from(user_id));
        payload.insert("duration".to_string(), serde_json::Value::String(duration.to_string()));
        payload.insert("cmsgId".to_string(), serde_json::Value::from(cmsg_id));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(453, 0, 1, 1, &enc, true, to)
    }
}
