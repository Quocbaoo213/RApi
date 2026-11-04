use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn create_calendar_schedule(&self, group_id: i64, title: &str, start_time_ms: i64, end_time_ms: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("createCalendarSchedule".to_string()));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        payload.insert("startTimeMs".to_string(), serde_json::Value::from(start_time_ms));
        payload.insert("endTimeMs".to_string(), serde_json::Value::from(end_time_ms));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1767, 0, 1, 1, &enc, true, to)
    }
}
