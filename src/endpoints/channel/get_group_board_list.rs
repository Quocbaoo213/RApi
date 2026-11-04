use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn get_group_board_list(&self, group_id: i64, page: i32, count: i32, last_id: i64, last_type: i32, from_time: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("getGroupBoardList".to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("page".to_string(), serde_json::Value::from(page));
        payload.insert("count".to_string(), serde_json::Value::from(count));
        payload.insert("last_id".to_string(), serde_json::Value::from(last_id));
        payload.insert("last_type".to_string(), serde_json::Value::from(last_type));
        payload.insert("fromTime".to_string(), serde_json::Value::from(from_time));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1703, 0, 1, 1, &enc, true, to)
    }
}
