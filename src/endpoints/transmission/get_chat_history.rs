use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn get_chat_history(&self, group_id: i64, count: i32, last_msg_id: i64, cli_msg_id: i64, direction: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("getChatHistory".to_string()));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("count".to_string(), serde_json::Value::from(count));
        payload.insert("lastMsgId".to_string(), serde_json::Value::from(last_msg_id));
        payload.insert("cliMsgId".to_string(), serde_json::Value::from(cli_msg_id));
        payload.insert("direction".to_string(), serde_json::Value::String(direction.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1853, 0, 1, 1, &enc, true, to)
    }
}
