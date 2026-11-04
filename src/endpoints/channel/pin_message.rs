use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn pin_message(&self, group_id: i64, pin_msg: &str, title: &str, cli_msg_id: i64, global_msg_id: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("pinMessage".to_string()));
        payload.insert("pinMsg".to_string(), serde_json::Value::String(pin_msg.to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        payload.insert("cliMsgId".to_string(), serde_json::Value::from(cli_msg_id));
        payload.insert("globalMsgId".to_string(), serde_json::Value::from(global_msg_id));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1752, 0, 1, 1, &enc, true, to)
    }
}
