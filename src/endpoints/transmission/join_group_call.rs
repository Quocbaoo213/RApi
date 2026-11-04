use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn join_group_call(&self, group_id: i64, user_id: i64, session: &str, call_type: i32, rtp_addr: &str, rtcp_addr: &str, cmsg_id: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("joinGroupCall".to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("session".to_string(), serde_json::Value::String(session.to_string()));
        payload.insert("userId".to_string(), serde_json::Value::from(user_id));
        payload.insert("callType".to_string(), serde_json::Value::from(call_type));
        payload.insert("userId".to_string(), serde_json::Value::from(user_id));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("rtpAddr".to_string(), serde_json::Value::String(rtp_addr.to_string()));
        payload.insert("rtcpAddr".to_string(), serde_json::Value::String(rtcp_addr.to_string()));
        payload.insert("cmsgId".to_string(), serde_json::Value::from(cmsg_id));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(451, 0, 1, 1, &enc, true, to)
    }
}
