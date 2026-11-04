use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn unpin_message(&self, group_id: i64, pin_id: i64, pin_time: i64, topic_type: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("unpinMessage".to_string()));
        payload.insert("pinId".to_string(), serde_json::Value::from(pin_id));
        payload.insert("pinTime".to_string(), serde_json::Value::from(pin_time));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("topicType".to_string(), serde_json::Value::from(topic_type));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(241, 0, 1, 1, &enc, true, to)
    }
}
