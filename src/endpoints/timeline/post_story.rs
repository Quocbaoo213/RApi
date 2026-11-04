use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TimelineEndpoint;

impl TimelineEndpoint {
    pub fn post_story(&self, media_url: &str, caption: &str, duration_sec: i32, privacy: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("post_story".to_string()));
        payload.insert("mediaUrl".to_string(), serde_json::Value::String(media_url.to_string()));
        payload.insert("caption".to_string(), serde_json::Value::String(caption.to_string()));
        payload.insert("durationSec".to_string(), serde_json::Value::from(duration_sec));
        payload.insert("privacy".to_string(), serde_json::Value::from(privacy));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(392, 0, 1, 1, &enc, true, to)
    }
}
