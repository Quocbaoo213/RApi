use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_encode_d3;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn send_direct_message(&self, user_id: i64, text: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let (seq, cmsg, _) = self.ctx.connection.as_ref().map(|c| c.get_next_counters()).unwrap_or((-1, 1, 0));
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let mut d3_map = HashMap::new();
        d3_map.insert("message".to_string(), serde_json::Value::String(text.to_string()));
        d3_map.insert("targetId".to_string(), serde_json::Value::from(user_id));
        let plain = serde_json::to_vec(&d3_map).map_err(|e| e.to_string())?;
        let d3_xor = xor_encode_d3(&plain, uid, None);
        let mut prefix = Vec::new();
        prefix.extend_from_slice(&((user_id & 0xFFFFFFFF) as u32).to_le_bytes());
        prefix.push(3);
        prefix.extend_from_slice(&((cmsg & 0x7FFFFFFF) as u32).to_le_bytes());
        prefix.extend_from_slice(&(0u32).to_le_bytes());
        let mut inner = self.ctx.wrap_inner(113, 0, uid, &prefix, seq, 1, 1, 3, None);
        inner.extend_from_slice(&d3_xor);
        self.ctx.send_raw_packet(&inner, true, 113, to)
    }
}
