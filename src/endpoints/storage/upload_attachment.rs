use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::StorageEndpoint;

impl StorageEndpoint {
    pub fn upload_attachment(&self, group_id: i64, source: &str, thread_type: i32, ext: &str, as_cloud: &str, return_info: &str, media_type: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("upload_attachment".to_string()));
        payload.insert("source".to_string(), serde_json::Value::String(source.to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("thread_type".to_string(), serde_json::Value::from(thread_type));
        payload.insert("ext".to_string(), serde_json::Value::String(ext.to_string()));
        payload.insert("as_cloud".to_string(), serde_json::Value::String(as_cloud.to_string()));
        payload.insert("return_info".to_string(), serde_json::Value::String(return_info.to_string()));
        payload.insert("media_type".to_string(), serde_json::Value::from(media_type));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
