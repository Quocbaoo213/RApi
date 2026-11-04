use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn send_voip_feedback_rating(&self, call_id: i64, rating: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("sendVoipFeedbackRating".to_string()));
        payload.insert("callId".to_string(), serde_json::Value::from(call_id));
        payload.insert("rating".to_string(), serde_json::Value::String(rating.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(420, 0, 1, 1, &enc, true, to)
    }
}
