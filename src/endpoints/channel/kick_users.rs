use std::time::Duration;
use crate::core::contract::response::ApiResult;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn kick_users(&self, group_id: i64, members: &[i64], timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(5));
        let (seq, _, _) = self.ctx.connection.as_ref().map(|c| c.get_next_counters()).unwrap_or((-1, 0, 0));
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let mut buf = Vec::new();
        buf.extend_from_slice(&((group_id & 0xFFFFFFFF) as u32).to_le_bytes());
        buf.push(0);
        buf.extend_from_slice(&(members.len() as u32).to_le_bytes());
        for &m in members {
            buf.extend_from_slice(&((m & 0xFFFFFFFF) as u32).to_le_bytes());
        }
        let inner = self.ctx.wrap_inner(234, 0, uid, &buf, seq, 0, 1, 3, None);
        self.ctx.send_raw_packet(&inner, true, 234, to)
    }
}
