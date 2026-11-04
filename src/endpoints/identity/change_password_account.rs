use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::IdentityEndpoint;

impl IdentityEndpoint {
    pub fn change_password_account(&self, new_password: &str, old_password: &str, phone_number: i32, update_config: &str, config_path: &str, session_file: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("ChangePasswordAccount".to_string()));
        payload.insert("new_password".to_string(), serde_json::Value::String(new_password.to_string()));
        payload.insert("old_password".to_string(), serde_json::Value::String(old_password.to_string()));
        payload.insert("phone_number".to_string(), serde_json::Value::from(phone_number));
        payload.insert("update_config".to_string(), serde_json::Value::String(update_config.to_string()));
        payload.insert("config_path".to_string(), serde_json::Value::String(config_path.to_string()));
        payload.insert("session_file".to_string(), serde_json::Value::String(session_file.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
