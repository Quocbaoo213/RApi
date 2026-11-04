use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::IdentityEndpoint;

impl IdentityEndpoint {
    pub fn find_nearby_friends(&self, gender: i32, min_age: i32, max_age: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("find_nearby_friends".to_string()));
        payload.insert("gender".to_string(), serde_json::Value::from(gender));
        payload.insert("minAge".to_string(), serde_json::Value::from(min_age));
        payload.insert("maxAge".to_string(), serde_json::Value::from(max_age));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
