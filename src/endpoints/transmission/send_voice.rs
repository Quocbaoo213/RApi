use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn send_voice(&self, group_id: i64, voice_url: &str, file_size: i64, ttl: i32, duration_ms: i32, waveform: &[serde_json::Value], timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("send_voice".to_string()));
        payload.insert("voiceUrl".to_string(), serde_json::Value::String(voice_url.to_string()));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("fileSize".to_string(), serde_json::Value::from(file_size));
        payload.insert("ttl".to_string(), serde_json::Value::from(ttl));
        payload.insert("durationMs".to_string(), serde_json::Value::from(duration_ms));
        payload.insert("waveform".to_string(), serde_json::to_value(waveform).unwrap_or_default());
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
