use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn send_split_bill(&self, group_id: i64, money: &str, reason: &str, card_id: i64, native: &str, wait_ack: bool, wait_sync: bool, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("sendSplitBill".to_string()));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("money".to_string(), serde_json::Value::String(money.to_string()));
        payload.insert("reason".to_string(), serde_json::Value::String(reason.to_string()));
        payload.insert("cardId".to_string(), serde_json::Value::from(card_id));
        payload.insert("groupId".to_string(), serde_json::Value::from(group_id));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("native".to_string(), serde_json::Value::String(native.to_string()));
        payload.insert("wait_ack".to_string(), serde_json::Value::Bool(wait_ack));
        payload.insert("wait_sync".to_string(), serde_json::Value::Bool(wait_sync));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(207, 0, 1, 1, &enc, true, to)
    }
}
