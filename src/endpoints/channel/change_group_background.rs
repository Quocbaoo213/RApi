use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn change_group_background(&self, group_id: i64, bg_url: &str, thumb_url: &str, title: &str, sub_id: i64, action_bar_color: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("changeGroupBackground".to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("bgUrl".to_string(), serde_json::Value::String(bg_url.to_string()));
        payload.insert("thumbUrl".to_string(), serde_json::Value::String(thumb_url.to_string()));
        payload.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        payload.insert("subId".to_string(), serde_json::Value::from(sub_id));
        payload.insert("actionBarColor".to_string(), serde_json::Value::String(action_bar_color.to_string()));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(207, 0, 1, 1, &enc, true, to)
    }
}
