use super::key::DEFAULT_XK;

pub fn xor_d3(data: &[u8], key: Option<&[u8]>) -> Vec<u8> {
    let k = key.unwrap_or(DEFAULT_XK);
    let k_len = k.len();
    if k_len == 0 {
        return data.to_vec();
    }
    data.iter()
        .enumerate()
        .map(|(i, &b)| b ^ k[i % k_len])
        .collect()
}

pub fn xor_encode_d3(plain: &[u8], uid: i64, xk: Option<&[u8]>) -> Vec<u8> {
    let k_src = xk.unwrap_or(DEFAULT_XK);
    let a4 = (plain.len() + 36) as u32;
    let le4 = a4.to_le_bytes();
    let ule = (uid as u32).to_le_bytes();
    let mut k = [0u8; 32];
    for i in 0..32 {
        k[i] = k_src[i % 32] ^ le4[i % 4] ^ ule[i % 4];
    }
    plain
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ k[i % 32])
        .collect()
}

pub fn xor_decode_d3(cipher: &[u8], uid: i64, xk: Option<&[u8]>) -> Vec<u8> {
    xor_encode_d3(cipher, uid, xk)
}

pub fn dekey(a4: usize, uid: i64, xk: Option<&[u8]>) -> [u8; 32] {
    let k_src = xk.unwrap_or(DEFAULT_XK);
    let ule = (uid as u32).to_le_bytes();
    let a4_le = (a4 as u32).to_le_bytes();
    let mut k1 = [0u8; 32];
    for i in 0..32 {
        k1[i] = k_src[i % 32] ^ a4_le[i % 4];
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = k1[i % 32] ^ ule[i % 4];
    }
    out
}

pub fn xor_enc(pt: &[u8], uid: i64, xk: Option<&[u8]>) -> Vec<u8> {
    let key = dekey(pt.len() + 23, uid, xk);
    pt.iter()
        .enumerate()
        .map(|(i, &b)| b ^ key[i % 32])
        .collect()
}
