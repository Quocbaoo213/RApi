use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn change_group_avatar_legacy(&self, group_id: i64, avatar_url: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let (seq, _, _) = self.ctx.connection.as_ref().map(|c| c.get_next_counters()).unwrap_or((-1, 0, 0));
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let mut payload = HashMap::new();
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("avatarUrl".to_string(), serde_json::Value::String(avatar_url.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let inner = self.ctx.wrap_inner(166, 0, uid, &b, seq, 0, 1, 3, None);
        self.ctx.send_raw_packet(&inner, true, 166, to)
    }
}
