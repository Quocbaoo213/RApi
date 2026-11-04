use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn sync_offline_messages(&self, from_msg_id: i64, count: i32, flag: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("syncOfflineMessages".to_string()));
        payload.insert("fromMsgId".to_string(), serde_json::Value::from(from_msg_id));
        payload.insert("count".to_string(), serde_json::Value::from(count));
        payload.insert("flag".to_string(), serde_json::Value::String(flag.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(96, 0, 1, 1, &enc, true, to)
    }
}
