use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn reply_message(&self, group_id: i64, message: &str, reply_msg: &str, ttl: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("replyMessage".to_string()));
        payload.insert("message".to_string(), serde_json::Value::String(message.to_string()));
        payload.insert("replyMsg".to_string(), serde_json::Value::String(reply_msg.to_string()));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("ttl".to_string(), serde_json::Value::from(ttl));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
