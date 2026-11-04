use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::StorageEndpoint;

impl StorageEndpoint {
    pub fn upload_binary(&self, chunk: &[serde_json::Value], chunk_index: i32, is_last: bool, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("upload_binary".to_string()));
        payload.insert("chunk".to_string(), serde_json::to_value(chunk).unwrap_or_default());
        payload.insert("chunkIndex".to_string(), serde_json::Value::from(chunk_index));
        payload.insert("isLast".to_string(), serde_json::Value::Bool(is_last));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(142, 0, 1, 1, &enc, true, to)
    }
}
