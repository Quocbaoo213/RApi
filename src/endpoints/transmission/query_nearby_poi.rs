use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn query_nearby_poi(&self, lat: f64, lon: f64, radius_meter: i32, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("query_nearby_poi".to_string()));
        payload.insert("lat".to_string(), serde_json::to_value(lat).unwrap_or_default());
        payload.insert("lon".to_string(), serde_json::to_value(lon).unwrap_or_default());
        payload.insert("radiusMeter".to_string(), serde_json::Value::from(radius_meter));
        let b = serde_json::to_vec(&payload).map_err(|e| e.to_string())?;
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&b, uid, None);
        self.ctx.send_socket_packet(100, 0, 1, 1, &enc, true, to)
    }
}
