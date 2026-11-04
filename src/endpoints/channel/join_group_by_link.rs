use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn join_group_by_link(&self, link_u_r_l: &str, vercode: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("join_group_by_link".to_string()));
        payload.insert("linkURL".to_string(), serde_json::Value::String(link_u_r_l.to_string()));
        payload.insert("vercode".to_string(), serde_json::Value::from(vercode));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(2000, 0, 1, 1, &enc, true, to)
    }
}
