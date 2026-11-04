use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TimelineEndpoint;

impl TimelineEndpoint {
    pub fn request_upload_url(&self, file_size: i64, file_name: &str, width: i64, height: i32, cli_file_id: i64, media_type: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("requestUploadUrl".to_string()));
        payload.insert("file_size".to_string(), serde_json::Value::from(file_size));
        payload.insert("file_name".to_string(), serde_json::Value::String(file_name.to_string()));
        payload.insert("width".to_string(), serde_json::Value::from(width));
        payload.insert("height".to_string(), serde_json::Value::from(height));
        payload.insert("cli_file_id".to_string(), serde_json::Value::from(cli_file_id));
        payload.insert("media_type".to_string(), serde_json::Value::from(media_type));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1604, 0, 1, 1, &enc, true, to)
    }
}
