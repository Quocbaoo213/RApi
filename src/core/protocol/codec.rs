use flate2::read::{GzDecoder, ZlibDecoder};
use std::collections::HashMap;
use std::io::Read;

use crate::core::security::aes::decrypt_aes_gcm;
use crate::core::security::cipher::xor_enc;
use crate::core::security::key::DEFAULT_XK;
use super::header::{PacketHeader, INNER_HEADER_SIZE};
use super::frame::WireFrame;

#[derive(Debug, Clone)]
pub struct InboundPacket {
    pub header: Option<PacketHeader>,
    pub raw_params: Vec<u8>,
    pub data: Option<HashMap<String, serde_json::Value>>,
    pub error_code: i32,
    pub error_msg: String,
    pub raw_text: String,
}

fn try_decompress(b: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    if b.starts_with(b"\x1f\x8b") {
        let mut gz = GzDecoder::new(b);
        let mut out = Vec::new();
        gz.read_to_end(&mut out)?;
        return Ok(out);
    }
    let mut zlib = ZlibDecoder::new(b);
    let mut out = Vec::new();
    zlib.read_to_end(&mut out)?;
    Ok(out)
}

pub fn parse_compressed_json(body: &[u8], uid: i64) -> (Option<HashMap<String, serde_json::Value>>, String) {
    if body.len() < 2 {
        return (None, String::new());
    }
    let mut candidates = vec![body];
    if body.len() > 4 {
        candidates.push(&body[4..]);
    }
    if body.len() > 8 {
        candidates.push(&body[8..]);
    }

    for b in &candidates {
        if b.starts_with(b"\x1f\x8b") || b.starts_with(b"x\x9c") || b.starts_with(b"x\x01") || b.starts_with(b"x\xda") {
            if let Ok(decomp) = try_decompress(b) {
                if let Ok(js) = serde_json::from_slice::<HashMap<String, serde_json::Value>>(&decomp) {
                    let txt = String::from_utf8_lossy(&decomp).to_string();
                    return (Some(js), txt);
                }
            }
        } else if b.starts_with(b"{") || b.starts_with(b"[") {
            if let Ok(js) = serde_json::from_slice::<HashMap<String, serde_json::Value>>(b) {
                let txt = String::from_utf8_lossy(b).to_string();
                return (Some(js), txt);
            }
        }
    }

    if body.len() < 10 {
        return (None, String::new());
    }

    let ule = (uid as u32).to_le_bytes();
    let offsets = [27, 0, 18, -18, 36, -36, 1, -1, 2, -2, 4, -4, 8, -8, 12, -12];

    for b in &candidates {
        for &offset in &offsets {
            let a4 = (b.len() as isize + offset) as u32;
            let le4 = a4.to_le_bytes();
            let mut k = [0u8; 32];
            for i in 0..32 {
                k[i] = DEFAULT_XK[i % 32] ^ le4[i % 4] ^ ule[i % 4];
            }
            let dec: Vec<u8> = b.iter().enumerate().map(|(i, &x)| x ^ k[i % 32]).collect();
            if let Ok(decomp) = try_decompress(&dec) {
                let txt = String::from_utf8_lossy(&decomp).to_string();
                if let Ok(js) = serde_json::from_slice::<HashMap<String, serde_json::Value>>(&decomp) {
                    return (Some(js), txt);
                }
            }
        }
    }

    (None, String::new())
}

pub fn parse_incoming_frame(frame: &WireFrame, dk: &[u8], uid: i64) -> Result<Option<InboundPacket>, String> {
    if frame.body.is_empty() {
        return Ok(None);
    }
    let inner_data = if frame.body.len() >= 28 {
        decrypt_aes_gcm(&frame.body, dk).unwrap_or_else(|_| frame.body.clone())
    } else {
        frame.body.clone()
    };

    if inner_data.len() < INNER_HEADER_SIZE {
        return Ok(Some(InboundPacket {
            header: None,
            raw_params: inner_data,
            data: None,
            error_code: 0,
            error_msg: String::new(),
            raw_text: String::new(),
        }));
    }

    let hdr = PacketHeader::unpack(&inner_data[..INNER_HEADER_SIZE])?;
    let payload = &inner_data[INNER_HEADER_SIZE..];
    let (js, txt) = parse_compressed_json(payload, uid);

    let mut error_code = 0;
    let mut error_msg = String::new();

    if let Some(ref m) = js {
        if let Some(v) = m.get("error_code").and_then(|v| v.as_i64()) {
            error_code = v as i32;
        } else if let Some(v) = m.get("errorCode").and_then(|v| v.as_i64()) {
            error_code = v as i32;
        }
        if let Some(v) = m.get("error_message").and_then(|v| v.as_str()) {
            error_msg = v.to_string();
        } else if let Some(v) = m.get("errorMessage").and_then(|v| v.as_str()) {
            error_msg = v.to_string();
        }
    }

    Ok(Some(InboundPacket {
        header: Some(hdr),
        raw_params: payload.to_vec(),
        data: js,
        error_code,
        error_msg,
        raw_text: txt,
    }))
}

pub fn wrap_inner_packet(
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
    let enc_params = xor_enc(params, uid, None);
    let ck = ck_val.unwrap_or_else(|| {
        crate::core::security::checksum::compute_inner_checksum(cmd, sub, seq, uid, bb, ty, ver)
    });
    let hdr = PacketHeader {
        ck,
        bb: bb as u8,
        ty: ty as u8,
        seq,
        uid: (uid & 0xFFFFFFFF) as u32,
        ver: ver as u8,
        cmd: cmd as u16,
        sub: sub as u8,
    };
    let mut out = hdr.pack();
    out.extend_from_slice(&enc_params);
    out
}
