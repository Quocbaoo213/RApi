use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn join_by_link(&self, group_id: i64, link_or_code: i32, msg: &str, source: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("joinByLink".to_string()));
        payload.insert("link_or_code".to_string(), serde_json::Value::from(link_or_code));
        payload.insert("msg".to_string(), serde_json::Value::String(msg.to_string()));
        payload.insert("source".to_string(), serde_json::Value::String(source.to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
