use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::core::contract::response::ApiResult;
use crate::core::network::http::HttpClient;
use crate::core::protocol::codec::wrap_inner_packet;
use crate::core::protocol::header::PacketHeader;
use crate::core::security::checksum::compute_checksum;
use crate::core::transport::connection::SocketConnection;

#[derive(Clone)]
pub struct BaseContext {
    pub connection: Option<Arc<SocketConnection>>,
    pub http: Arc<HttpClient>,
}

impl BaseContext {
    pub fn new(connection: Option<Arc<SocketConnection>>) -> Self {
        Self {
            connection,
            http: Arc::new(HttpClient::new()),
        }
    }

    pub fn send_socket_packet(
        &self,
        cmd: u16,
        sub: u8,
        bb: u8,
        ty: u8,
        payload: &[u8],
        wait_ack: bool,
        timeout: Duration,
    ) -> Result<ApiResult, String> {
        let conn = match &self.connection {
            Some(c) => c,
            None => return Ok(ApiResult::error(-1, "client is nil")),
        };
        if !conn.is_connected() && !conn.connect() {
            return Ok(ApiResult::error(-1, "socket not connected"));
        }

        let (seq, _, _) = conn.get_next_counters();
        let ck = compute_checksum(
            cmd as i32,
            sub as i32,
            seq,
            conn.uid,
            0,
            0,
            0,
            bb as i32,
            ty as i32,
            3,
        );
        let hdr = PacketHeader {
            ck,
            bb,
            ty,
            seq,
            uid: (conn.uid & 0xFFFFFFFF) as u32,
            ver: 3,
            cmd,
            sub,
        };

        let mut inner_packet = hdr.pack();
        inner_packet.extend_from_slice(payload);

        let parsed = conn.send_packet(&inner_packet, wait_ack, cmd, timeout)?;
        if !wait_ack {
            return Ok(ApiResult::success(None));
        }

        match parsed {
            Some(p) => {
                let mut em = p.error_msg;
                if em.is_empty() && p.error_code == 0 {
                    em = "successful".to_string();
                }
                Ok(ApiResult {
                    error_code: p.error_code,
                    error_message: em,
                    data: p.data,
                    raw_params: Some(p.raw_params),
                    success: p.error_code == 0,
                })
            }
            None => Ok(ApiResult::error(-2, "no response")),
        }
    }

    pub fn send_raw_packet(
        &self,
        inner_packet: &[u8],
        wait_ack: bool,
        cmd: u16,
        timeout: Duration,
    ) -> Result<ApiResult, String> {
        let conn = match &self.connection {
            Some(c) => c,
            None => return Ok(ApiResult::error(-1, "client is nil")),
        };
        if !conn.is_connected() && !conn.connect() {
            return Ok(ApiResult::error(-1, "socket not connected"));
        }

        let parsed = conn.send_packet(inner_packet, wait_ack, cmd, timeout)?;
        if !wait_ack {
            return Ok(ApiResult::success(None));
        }

        match parsed {
            Some(p) => {
                let mut em = p.error_msg;
                if em.is_empty() && p.error_code == 0 {
                    em = "successful".to_string();
                }
                Ok(ApiResult {
                    error_code: p.error_code,
                    error_message: em,
                    data: p.data,
                    raw_params: Some(p.raw_params),
                    success: p.error_code == 0,
                })
            }
            None => Ok(ApiResult::error(-2, "no response")),
        }
    }

    pub fn post_encrypted_http(
        &self,
        url: &str,
        params: &HashMap<String, serde_json::Value>,
        zcid: &str,
        api_key: Option<&str>,
        secret: Option<&str>,
    ) -> Result<ApiResult, String> {
        self.http.post_encrypted(url, params, zcid, api_key, secret)
    }

    pub fn get_json(
        &self,
        url: &str,
        headers: &HashMap<String, String>,
    ) -> Result<ApiResult, String> {
        self.http.get_json(url, headers)
    }

    pub fn wrap_inner(
        &self,
        cmd: i32,
        sub: i32,
        uid: i64,
        params: &[u8],
        seq: i32,
        bb: i32,
        ty: i32,
        ver: i32,
        ck_val: Option<u32>,
    ) -> Vec<u8> {
        wrap_inner_packet(cmd, sub, uid, params, seq, bb, ty, ver, ck_val)
    }
}
