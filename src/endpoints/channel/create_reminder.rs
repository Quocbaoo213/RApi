use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn create_reminder(&self, group_id: i64, title: &str, start_time_ms: i64, end_time_ms: i64, emoji: &str, color: &str, only_me: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("createReminder".to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        payload.insert("start_time_ms".to_string(), serde_json::Value::from(start_time_ms));
        payload.insert("end_time_ms".to_string(), serde_json::Value::from(end_time_ms));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("emoji".to_string(), serde_json::Value::String(emoji.to_string()));
        payload.insert("color".to_string(), serde_json::Value::String(color.to_string()));
        payload.insert("only_me".to_string(), serde_json::Value::String(only_me.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1764, 0, 1, 1, &enc, true, to)
    }
}
