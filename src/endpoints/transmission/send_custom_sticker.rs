use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn send_custom_sticker(&self, group_id: i64, sticker_i_d: i32, cat_i_d: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("custom_sticker".to_string()));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("stickerID".to_string(), serde_json::Value::from(sticker_i_d));
        payload.insert("catID".to_string(), serde_json::Value::from(cat_i_d));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
