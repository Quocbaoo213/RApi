use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn send_group_event_sub57(&self, group_id: i64, message: &str, wait_ack: bool, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("sendGroupEventSub57".to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("message".to_string(), serde_json::Value::String(message.to_string()));
        payload.insert("wait_ack".to_string(), serde_json::Value::Bool(wait_ack));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(207, 0, 1, 1, &enc, true, to)
    }
}
