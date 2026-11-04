pub const INNER_HEADER_SIZE: usize = 18;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PacketHeader {
    pub ck: u32,
    pub bb: u8,
    pub ty: u8,
    pub seq: i32,
    pub uid: u32,
    pub ver: u8,
    pub cmd: u16,
    pub sub: u8,
}

impl PacketHeader {
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(INNER_HEADER_SIZE);
        buf.extend_from_slice(&self.ck.to_le_bytes());
        buf.push(self.bb);
        buf.push(self.ty);
        buf.extend_from_slice(&self.seq.to_le_bytes());
        buf.extend_from_slice(&self.uid.to_le_bytes());
        buf.push(self.ver);
        buf.extend_from_slice(&self.cmd.to_le_bytes());
        buf.push(self.sub);
        buf
    }

    pub fn unpack(data: &[u8]) -> Result<Self, String> {
        if data.len() < INNER_HEADER_SIZE {
            return Err("data length insufficient for inner header".to_string());
        }
        let ck = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let bb = data[4];
        let ty = data[5];
        let seq = i32::from_le_bytes([data[6], data[7], data[8], data[9]]);
        let uid = u32::from_le_bytes([data[10], data[11], data[12], data[13]]);
        let ver = data[14];
        let cmd = u16::from_le_bytes([data[15], data[16]]);
        let sub = data[17];
        Ok(Self {
            ck,
            bb,
            ty,
            seq,
            uid,
            ver,
            cmd,
            sub,
        })
    }
}
