use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TimelineEndpoint;

impl TimelineEndpoint {
    pub fn edit_feed(&self, feed_id: &str, user_id: i64, caption: &str, privacy_mode: i32, allow_comment: bool, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("editFeed".to_string()));
        payload.insert("feedId".to_string(), serde_json::Value::String(feed_id.to_string()));
        payload.insert("caption".to_string(), serde_json::Value::String(caption.to_string()));
        payload.insert("userId".to_string(), serde_json::Value::from(user_id));
        payload.insert("privacy_mode".to_string(), serde_json::Value::from(privacy_mode));
        payload.insert("allow_comment".to_string(), serde_json::Value::Bool(allow_comment));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(2034, 0, 1, 1, &enc, true, to)
    }
}
