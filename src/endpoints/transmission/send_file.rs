use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn send_file(&self, group_id: i64, file_url: &str, file_name: &str, file_size: i64, file_ext: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("send_file".to_string()));
        payload.insert("fileUrl".to_string(), serde_json::Value::String(file_url.to_string()));
        payload.insert("fileName".to_string(), serde_json::Value::String(file_name.to_string()));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("fileSize".to_string(), serde_json::Value::from(file_size));
        payload.insert("fileExt".to_string(), serde_json::Value::String(file_ext.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
