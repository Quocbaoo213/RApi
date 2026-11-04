use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TimelineEndpoint;

impl TimelineEndpoint {
    pub fn create_poll(&self, question: &str, options: &[serde_json::Value], group_id: i64, expired_time: i64, pin_act: &str, multi_choices: &str, allow_add_new_option: bool, hide_vote_preview: i64, is_anonymous: bool, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("createPoll".to_string()));
        payload.insert("question".to_string(), serde_json::Value::String(question.to_string()));
        payload.insert("options".to_string(), serde_json::to_value(options).unwrap_or_default());
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("expiredTime".to_string(), serde_json::Value::from(expired_time));
        payload.insert("pinAct".to_string(), serde_json::Value::String(pin_act.to_string()));
        payload.insert("multiChoices".to_string(), serde_json::Value::String(multi_choices.to_string()));
        payload.insert("allowAddNewOption".to_string(), serde_json::Value::Bool(allow_add_new_option));
        payload.insert("hideVotePreview".to_string(), serde_json::Value::from(hide_vote_preview));
        payload.insert("isAnonymous".to_string(), serde_json::Value::Bool(is_anonymous));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1640, 0, 1, 1, &enc, true, to)
    }
}
