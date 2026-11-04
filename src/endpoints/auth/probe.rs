use std::collections::HashMap;
use std::time::Duration;

use crate::core::contract::response::ApiResult;
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn probe_network(&self, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let _to = timeout.unwrap_or(Duration::from_secs(5));
        let headers = HashMap::new();
        let url = "https://id.zalo.me/account/probe";
        self.ctx.get_json(url, &headers)
    }
}
