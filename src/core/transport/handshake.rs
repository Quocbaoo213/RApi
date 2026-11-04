use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::Engine;
use rand::RngCore;
use x25519_dalek::{PublicKey, StaticSecret};

use crate::core::security::checksum::CKSUM_MAGIC;
use crate::core::security::key::DEFAULT_XK;

pub fn build_p110(sk_str: &str, uid: i64, vercode: u32) -> Vec<u8> {
    let s = sk_str.as_bytes();
    let mut inner = Vec::new();
    inner.push(1);
    inner.extend_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]);
    inner.extend_from_slice(&[0, 0]);
    inner.extend_from_slice(&vercode.to_le_bytes());
    inner.push(2);
    inner.extend_from_slice(&(19u16).to_le_bytes());
    inner.extend_from_slice(b"samsung_SM-J610F_11");
    inner.extend_from_slice(&(6u16).to_le_bytes());
    inner.extend_from_slice(b"000000");
    inner.extend_from_slice(&[0u8; 6]);
    inner.extend_from_slice(&(s.len() as u16).to_le_bytes());
    inner.extend_from_slice(s);

    let le_t = (133u32).to_le_bytes();
    let le_m = ((uid & 0xFFFFFFFF) as u32).to_le_bytes();
    let mut key = [0u8; 32];
    for i in 0..32 {
        key[i] = DEFAULT_XK[i % 32] ^ le_t[i % 4] ^ le_m[i % 4];
    }

    let mut out = vec![0u8; 110];
    for i in 0..110.min(inner.len()) {
        out[i] = inner[i] ^ key[i % 32];
    }
    out
}

pub fn build_authen_body(sk_str: &str, uid: i64, ctr: u32) -> Vec<u8> {
    let s = (((uid & 0xFFFFFFFF) as u32)
        .wrapping_add(ctr)
        .wrapping_add(1)
        .wrapping_add(3)
        .wrapping_add(5)
        .wrapping_add(1)
        .wrapping_add(1))
        & 0xFFFFFFFF;
    let cs = s ^ CKSUM_MAGIC;

    let mut buf = Vec::new();
    buf.extend_from_slice(&cs.to_le_bytes());
    buf.extend_from_slice(&[1, 1]);
    buf.extend_from_slice(&ctr.to_le_bytes());
    buf.extend_from_slice(&((uid & 0xFFFFFFFF) as u32).to_le_bytes());
    buf.extend_from_slice(&[3, 1, 0, 5]);
    buf.extend_from_slice(&build_p110(sk_str, uid, 260801901));
    buf
}

pub fn build_prov_frame(
    session_key: &str,
    dk: &[u8],
    ksid: &str,
    server_pub_key_b64: &str,
    uid: i64,
) -> Result<Vec<u8>, String> {
    let s = server_pub_key_b64.trim();
    let srv_pub = if let Ok(b) = base64::engine::general_purpose::STANDARD.decode(s) {
        if b.len() == 32 {
            b
        } else if s.len() == 32 {
            s.as_bytes().to_vec()
        } else {
            return Err("invalid server public key".to_string());
        }
    } else if s.len() == 32 {
        s.as_bytes().to_vec()
    } else {
        return Err("invalid server public key".to_string());
    };

    let body = build_authen_body(session_key, uid, 0xFFFFFFFF);
    let mut iv = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut iv);

    let cipher = Aes256Gcm::new_from_slice(dk).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(&iv);
    let sealed = cipher.encrypt(nonce, body.as_ref()).map_err(|e| e.to_string())?;
    let split_idx = sealed.len() - 16;
    let ct = &sealed[..split_idx];
    let tag = &sealed[split_idx..];

    let mut inner = Vec::new();
    inner.extend_from_slice(ct);
    inner.extend_from_slice(tag);
    inner.extend_from_slice(&iv);
    inner.extend_from_slice(ksid.as_bytes());
    inner.push(0x10);

    let mut srv_pub_arr = [0u8; 32];
    srv_pub_arr.copy_from_slice(&srv_pub);
    let remote_pub = PublicKey::from(srv_pub_arr);

    let mut secret_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut secret_bytes);
    let priv_key = StaticSecret::from(secret_bytes);
    let pub_key = PublicKey::from(&priv_key);
    let shared = priv_key.diffie_hellman(&remote_pub);

    let mut eiv = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut eiv);

    let sh_cipher = Aes256Gcm::new_from_slice(shared.as_bytes()).map_err(|e| e.to_string())?;
    let enonce = Nonce::from_slice(&eiv);
    let outer = sh_cipher.encrypt(enonce, inner.as_ref()).map_err(|e| e.to_string())?;

    let mut payload = Vec::new();
    payload.extend_from_slice(&outer);
    payload.extend_from_slice(&eiv);
    payload.extend_from_slice(pub_key.as_bytes());
    payload.extend_from_slice(&srv_pub);

    let mut res = Vec::with_capacity(5 + payload.len());
    let total_len = (5 + payload.len()) as u32;
    res.extend_from_slice(&total_len.to_le_bytes());
    res.push(8);
    res.extend_from_slice(&payload);
    Ok(res)
}
