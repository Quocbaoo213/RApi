use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::IdentityEndpoint;

impl IdentityEndpoint {
    pub fn get_contact_timeline_state(&self, page: i32, page_size: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("get_contact_timeline_state".to_string()));
        payload.insert("page".to_string(), serde_json::Value::from(page));
        payload.insert("pageSize".to_string(), serde_json::Value::from(page_size));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1207, 0, 1, 1, &enc, true, to)
    }
}
