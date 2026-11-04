use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TimelineEndpoint;

impl TimelineEndpoint {
    pub fn vote_poll(&self, poll_id: i64, option_ids: i64, group_id: i64, new_options: &[serde_json::Value], timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("votePoll".to_string()));
        payload.insert("pollId".to_string(), serde_json::Value::from(poll_id));
        payload.insert("optionIds".to_string(), serde_json::Value::from(option_ids));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("new_options".to_string(), serde_json::to_value(new_options).unwrap_or_default());
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1645, 0, 1, 1, &enc, true, to)
    }
}
