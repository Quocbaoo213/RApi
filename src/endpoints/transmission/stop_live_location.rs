use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn stop_live_location(&self, group_id: i64, live_location_id: i64, lat: &str, lon: &str, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("stopLiveLocation".to_string()));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("liveLocationId".to_string(), serde_json::Value::from(live_location_id));
        payload.insert("lat".to_string(), serde_json::Value::String(lat.to_string()));
        payload.insert("lon".to_string(), serde_json::Value::String(lon.to_string()));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(1413, 0, 1, 1, &enc, true, to)
    }
}
