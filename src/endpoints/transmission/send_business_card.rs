use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn send_business_card(&self, group_id: i64, user_id: i64, qr_code_url: i32, phone: &str, ttl: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("sendBusinessCard".to_string()));
        payload.insert("userId".to_string(), serde_json::Value::from(user_id));
        payload.insert("qrCodeUrl".to_string(), serde_json::Value::from(qr_code_url));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("phone".to_string(), serde_json::Value::String(phone.to_string()));
        payload.insert("ttl".to_string(), serde_json::Value::from(ttl));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
