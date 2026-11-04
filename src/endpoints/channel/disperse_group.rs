use std::time::Duration;
use crate::core::contract::response::ApiResult;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn disperse_group(&self, group_id: i64, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let (seq, _, _) = self.ctx.connection.as_ref().map(|c| c.get_next_counters()).unwrap_or((-1, 0, 0));
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let mut buf = Vec::new();
        buf.push(1);
        buf.extend_from_slice(&(260802903u32).to_le_bytes());
        buf.extend_from_slice(&((group_id & 0xFFFFFFFF) as u32).to_le_bytes());
        let inner = self.ctx.wrap_inner(242, 0, uid, &buf, seq, 0, 1, 3, None);
        self.ctx.send_raw_packet(&inner, true, 242, to)
    }
}
