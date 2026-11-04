use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::StorageEndpoint;

impl StorageEndpoint {
    pub fn upload_voice_stream(&self, group_id: i64, audio_bytes: &[serde_json::Value], thread_type: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("upload_voice_stream".to_string()));
        payload.insert("audio_bytes".to_string(), serde_json::to_value(audio_bytes).unwrap_or_default());
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("thread_type".to_string(), serde_json::Value::from(thread_type));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1, 3, 1, 1, &enc, true, to)
    }
}
