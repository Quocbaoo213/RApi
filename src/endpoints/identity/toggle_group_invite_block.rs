use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::IdentityEndpoint;

impl IdentityEndpoint {
    pub fn toggle_group_invite_block(&self, enable: bool, setting_id: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("toggleGroupInviteBlock".to_string()));
        payload.insert("enable".to_string(), serde_json::Value::Bool(enable));
        payload.insert("setting_id".to_string(), serde_json::Value::from(setting_id));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1964, 0, 1, 1, &enc, true, to)
    }
}
