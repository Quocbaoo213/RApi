use std::collections::HashMap;
use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::security::cipher::xor_enc;
use super::TransmissionEndpoint;

impl TransmissionEndpoint {
    pub fn send_product_catalog(&self, group_id: i64, product_id: i64, catalog_id: i64, seller_id: i64, title: &str, price: &str, thumb_url: &str, href: &str, native: &str, wait_ack: bool, wait_sync: bool, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let mut payload = HashMap::new();
        payload.insert("action".to_string(), serde_json::Value::String("sendProductCatalog".to_string()));
        payload.insert("threadId".to_string(), serde_json::Value::from(group_id));
        payload.insert("productId".to_string(), serde_json::Value::from(product_id));
        payload.insert("catalogId".to_string(), serde_json::Value::from(catalog_id));
        payload.insert("sellerId".to_string(), serde_json::Value::from(seller_id));
        payload.insert("title".to_string(), serde_json::Value::String(title.to_string()));
        payload.insert("price".to_string(), serde_json::Value::String(price.to_string()));
        payload.insert("thumbUrl".to_string(), serde_json::Value::String(thumb_url.to_string()));
        payload.insert("href".to_string(), serde_json::Value::String(href.to_string()));
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
