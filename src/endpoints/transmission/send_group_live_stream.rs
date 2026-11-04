use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn send_group_live_stream(&self, group_id: i64, video_url: i64, thumb_url: &str, title: &str, description: &str, sub_id: i64, action: &str, thumb_width: i64, thumb_height: i32, stream_id: i64, is_live_ended: bool, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("send_group_live_stream".to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("videoUrl".to_string(), serde_json::Value::from(video_url));
        payload.insert("thumbUrl".to_string(), serde_json::Value::String(thumb_url.to_string()));
        payload.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        payload.insert("description".to_string(), serde_json::Value::String(description.to_string()));
        payload.insert("subId".to_string(), serde_json::Value::from(sub_id));
        payload.insert("action".to_string(), serde_json::Value::String(action.to_string()));
        payload.insert("thumbWidth".to_string(), serde_json::Value::from(thumb_width));
        payload.insert("thumbHeight".to_string(), serde_json::Value::from(thumb_height));
        payload.insert("streamId".to_string(), serde_json::Value::from(stream_id));
        payload.insert("isLiveEnded".to_string(), serde_json::Value::Bool(is_live_ended));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(207, 0, 1, 1, &enc, true, to)
    }
}
