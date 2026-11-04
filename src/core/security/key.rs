use base64::Engine;

pub const DEFAULT_XK: &[u8; 32] = b"P1BL2G5I7NJD2H3JAUD554G7645PH54F";
pub const DEFAULT_BASE_KEY: &str = "d902f34a77e3084b9034eb0caee3e019";
pub const DEFAULT_SECRET: &str = "8cd0fc6c58e88e78d62db52f0e093367";

pub fn key32(zcid: &str) -> Result<Vec<u8>, String> {
    let z = zcid.trim().as_bytes();
    if z.len() < 96 {
        return Err("zcid length too short".to_string());
    }
    let mut part1 = vec![0u8; 16];
    for i in 0..16 {
        part1[i] = z[i * 2];
    }
    let mut part2 = vec![0u8; 16];
    for i in 0..16 {
        part2[i] = z[z.len() - 1 - 2 * i];
    }
    part1.extend_from_slice(&part2);
    Ok(part1)
}

pub fn derive_crypt_key_from_dk(dk: &[u8]) -> Result<Vec<u8>, String> {
    if dk.len() < 16 {
        return Err("invalid dk length".to_string());
    }
    let mask = b"P1BL2G5I7NJD2H3J";
    let mut out = vec![0u8; 16];
    for i in 0..16 {
        out[i] = dk[i] ^ mask[i];
    }
    Ok(out)
}

pub fn parse_dk(dk_str: &str) -> Result<Vec<u8>, String> {
    let s = dk_str.trim();
    if let Ok(b) = hex::decode(s) {
        if !b.is_empty() {
            return Ok(b);
        }
    }
    if let Ok(b) = base64::engine::general_purpose::STANDARD.decode(s) {
        if !b.is_empty() {
            return Ok(b);
        }
    }
    Ok(s.as_bytes().to_vec())
}
