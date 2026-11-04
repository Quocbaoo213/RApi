use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::IdentityEndpoint;

impl IdentityEndpoint {
    pub fn change_account_avatar(&self, file_path: &str, width: i64, height: i32, language: &str, size: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("changeAccountAvatar".to_string()));
        payload.insert("filePath".to_string(), serde_json::Value::String(file_path.to_string()));
        payload.insert("width".to_string(), serde_json::Value::from(width));
        payload.insert("height".to_string(), serde_json::Value::from(height));
        payload.insert("language".to_string(), serde_json::Value::String(language.to_string()));
        payload.insert("size".to_string(), serde_json::Value::from(size));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1, 3, 1, 1, &enc, true, to)
    }
}
