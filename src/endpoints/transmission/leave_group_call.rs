use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn leave_group_call(&self, group_id: i64, user_id: i64, session: &str, rtp_addr: &str, servers: i32, duration: &str, cur_state: &str, cmsg_id: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("leaveGroupCall".to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("session".to_string(), serde_json::Value::String(session.to_string()));
        payload.insert("userId".to_string(), serde_json::Value::from(user_id));
        payload.insert("rtpAddr".to_string(), serde_json::Value::String(rtp_addr.to_string()));
        payload.insert("servers".to_string(), serde_json::Value::from(servers));
        payload.insert("duration".to_string(), serde_json::Value::String(duration.to_string()));
        payload.insert("curState".to_string(), serde_json::Value::String(cur_state.to_string()));
        payload.insert("cmsgId".to_string(), serde_json::Value::from(cmsg_id));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(459, 0, 1, 1, &enc, true, to)
    }
}
