use crate::core::security::aes::encrypt_aes_gcm;

#[derive(Debug, Clone)]
pub struct WireFrame {
    pub total_len: u32,
    pub frame_type: u8,
    pub body: Vec<u8>,
}

impl WireFrame {
    pub fn pack(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(5 + self.body.len());
        let total = (5 + self.body.len()) as u32;
        out.extend_from_slice(&total.to_le_bytes());
        out.push(self.frame_type);
        out.extend_from_slice(&self.body);
        out
    }

    pub fn unpack(buf: &[u8]) -> Result<Option<(Self, usize)>, String> {
        if buf.len() < 5 {
            return Ok(None);
        }
        let total_len = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
        if total_len < 5 {
            return Err("invalid outer frame total length".to_string());
        }
        if buf.len() < total_len {
            return Ok(None);
        }
        let frame = Self {
            total_len: total_len as u32,
            frame_type: buf[4],
            body: buf[5..total_len].to_vec(),
        };
        Ok(Some((frame, total_len)))
    }

    pub fn build(inner_packet: &[u8], dk: &[u8]) -> Result<Vec<u8>, String> {
        let enc_body = encrypt_aes_gcm(inner_packet, dk, None)?;
        let frame = Self {
            total_len: (5 + enc_body.len()) as u32,
            frame_type: 1,
            body: enc_body,
        };
        Ok(frame.pack())
    }
}
