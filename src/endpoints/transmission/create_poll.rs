use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn create_poll(&self, group_id: i64, question: &str, options: &[serde_json::Value], expired_time: i64, multi_choice: &str, allow_add_option: bool, hide_vote: i64, hide_voters: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("createPoll".to_string()));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("question".to_string(), serde_json::Value::String(question.to_string()));
        payload.insert("options".to_string(), serde_json::to_value(options).unwrap_or_default());
        payload.insert("expiredTime".to_string(), serde_json::Value::from(expired_time));
        payload.insert("multiChoice".to_string(), serde_json::Value::String(multi_choice.to_string()));
        payload.insert("allowAddOption".to_string(), serde_json::Value::Bool(allow_add_option));
        payload.insert("hideVote".to_string(), serde_json::Value::from(hide_vote));
        payload.insert("hideVoters".to_string(), serde_json::Value::from(hide_voters));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
