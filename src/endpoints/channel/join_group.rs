use std::time::Duration;
use crate::core::contract::response::ApiResult;
use crate::core::protocol::opcode::DEFAULT_VERCODE;
use crate::core::security::cipher::xor_enc;
use super::ChannelEndpoint;

impl ChannelEndpoint {
    pub fn join_group(&self, group_id: i64, vercode: Option<u32>, timeout: Option<Duration>) -> Result<ApiResult, String> {
        let to = timeout.unwrap_or(Duration::from_secs(4));
        let vc = vercode.unwrap_or(DEFAULT_VERCODE);
        let mut buf = Vec::new();
        buf.push(1);
        buf.extend_from_slice(&vc.to_le_bytes());
        buf.extend_from_slice(&((group_id & 0xFFFFFFFF) as u32).to_le_bytes());
        let uid = self.ctx.connection.as_ref().map(|c| c.uid).unwrap_or(0);
        let enc = xor_enc(&buf, uid, None);
        self.ctx.send_socket_packet(246, 0, 1, 1, &enc, true, to)
    }
}
