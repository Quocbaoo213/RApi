use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TimelineEndpoint;

impl TimelineEndpoint {
    pub fn post_feed_video(&self, video: i64, caption: &str, width: i64, height: i32, duration: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("postFeedVideo".to_string()));
        payload.insert("video".to_string(), serde_json::Value::from(video));
        payload.insert("caption".to_string(), serde_json::Value::String(caption.to_string()));
        payload.insert("width".to_string(), serde_json::Value::from(width));
        payload.insert("height".to_string(), serde_json::Value::from(height));
        payload.insert("duration".to_string(), serde_json::Value::String(duration.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1606, 0, 1, 1, &enc, true, to)
    }
}
