use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::StorageEndpoint;

impl StorageEndpoint {
    pub fn download_chunked(&self, url: &str, chunk_size: i64, max_retries: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("download_chunked".to_string()));
        payload.insert("url".to_string(), serde_json::Value::String(url.to_string()));
        payload.insert("chunk_size".to_string(), serde_json::Value::from(chunk_size));
        payload.insert("max_retries".to_string(), serde_json::Value::String(max_retries.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
