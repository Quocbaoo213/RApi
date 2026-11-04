use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::StorageEndpoint;

impl StorageEndpoint {
    pub fn request_upload_u_r_l(&self, file_type: i32, file_size: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("request_upload".to_string()));
        payload.insert("fileType".to_string(), serde_json::Value::from(file_type));
        payload.insert("fileSize".to_string(), serde_json::Value::from(file_size));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
