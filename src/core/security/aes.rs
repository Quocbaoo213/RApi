use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use cbc::cipher::block_padding::NoPadding;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use md5::{Digest, Md5};
use rand::RngCore;
use std::collections::HashMap;

use super::key::{key32, DEFAULT_BASE_KEY, DEFAULT_SECRET};

type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;

pub fn encrypt_aes_gcm(plaintext: &[u8], dk: &[u8], iv_opt: Option<&[u8]>) -> Result<Vec<u8>, String> {
    if dk.len() != 32 {
        return Err(format!("dk length must be 32, got {}", dk.len()));
    }
    let mut iv = [0u8; 12];
    if let Some(i) = iv_opt {
        if i.len() != 12 {
            return Err(format!("iv length must be 12, got {}", i.len()));
        }
        iv.copy_from_slice(i);
    } else {
        rand::thread_rng().fill_bytes(&mut iv);
    }
    let cipher = Aes256Gcm::new_from_slice(dk).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(&iv);
    let ciphertext_and_tag = cipher.encrypt(nonce, plaintext).map_err(|e| e.to_string())?;
    if ciphertext_and_tag.len() < 16 {
        return Err("ciphertext too short".to_string());
    }
    let split_idx = ciphertext_and_tag.len() - 16;
    let ct = &ciphertext_and_tag[..split_idx];
    let tag = &ciphertext_and_tag[split_idx..];
    let mut res = Vec::with_capacity(ct.len() + tag.len() + iv.len());
    res.extend_from_slice(ct);
    res.extend_from_slice(tag);
    res.extend_from_slice(&iv);
    Ok(res)
}

pub fn decrypt_aes_gcm(frame_body: &[u8], dk: &[u8]) -> Result<Vec<u8>, String> {
    if frame_body.len() < 28 {
        return Err("frame body too short".to_string());
    }
    if dk.len() != 32 {
        return Err(format!("dk length must be 32, got {}", dk.len()));
    }
    let iv = &frame_body[frame_body.len() - 12..];
    let tag = &frame_body[frame_body.len() - 28..frame_body.len() - 12];
    let ct = &frame_body[..frame_body.len() - 28];
    let mut combined = Vec::with_capacity(ct.len() + tag.len());
    combined.extend_from_slice(ct);
    combined.extend_from_slice(tag);
    let cipher = Aes256Gcm::new_from_slice(dk).map_err(|e| e.to_string())?;
    let nonce = Nonce::from_slice(iv);
    cipher.decrypt(nonce, combined.as_ref()).map_err(|e| e.to_string())
}

pub fn password_hash(phone: &str, password: &str, base_key: Option<&str>) -> Result<String, String> {
    let bk = base_key.unwrap_or(DEFAULT_BASE_KEY);
    let mut hasher = Md5::new();
    hasher.update(format!("{}{}", bk, phone).as_bytes());
    let h = hasher.finalize();
    let hex_str = hex::encode(h);
    let key_bytes = &hex_str.as_bytes()[5..21];
    let mut blk = [0u8; 16];
    let pwd_bytes = password.as_bytes();
    let copy_len = pwd_bytes.len().min(16);
    blk[..copy_len].copy_from_slice(&pwd_bytes[..copy_len]);
    use aes::cipher::{BlockEncrypt, KeyInit as AesKeyInit};
    let cipher = aes::Aes128::new_from_slice(key_bytes).map_err(|e| e.to_string())?;
    let mut block = aes::Block::from(blk);
    cipher.encrypt_block(&mut block);
    Ok(hex::encode(block.as_slice()))
}

pub fn compute_sig(data_hex: &str, api_key: &str, secret: Option<&str>) -> String {
    let s = secret.unwrap_or(DEFAULT_SECRET);
    let payload = format!("api_key={}data={}{}", api_key, data_hex, s);
    let mut hasher = Md5::new();
    hasher.update(payload.as_bytes());
    hex::encode(hasher.finalize())
}

pub fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    let pad_len = block_size - (data.len() % block_size);
    let mut padded = Vec::with_capacity(data.len() + pad_len);
    padded.extend_from_slice(data);
    padded.resize(data.len() + pad_len, pad_len as u8);
    padded
}

pub fn pkcs7_unpad(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Err("empty data".to_string());
    }
    let pad_len = data[data.len() - 1] as usize;
    if pad_len == 0 || pad_len > 16 || pad_len > data.len() {
        return Ok(data.to_vec());
    }
    for &b in &data[data.len() - pad_len..] {
        if b as usize != pad_len {
            return Ok(data.to_vec());
        }
    }
    Ok(data[..data.len() - pad_len].to_vec())
}

pub fn encrypt_http_data(params: &HashMap<String, serde_json::Value>, zcid: &str) -> Result<String, String> {
    let k = key32(zcid)?;
    let key16 = if k.len() >= 16 { &k[..16] } else { return Err("key32 too short".to_string()); };
    let plain = serde_json::to_vec(params).map_err(|e| e.to_string())?;
    let mut buffer = pkcs7_pad(&plain, 16);
    let iv = [0u8; 16];
    let cipher = Aes128CbcEnc::new_from_slices(key16, &iv).map_err(|e| e.to_string())?;
    let blen = buffer.len(); cipher.encrypt_padded_mut::<NoPadding>(&mut buffer, blen).map_err(|e| format!("{:?}", e))?;
    Ok(hex::encode(buffer).to_uppercase())
}

pub fn decrypt_http_data(data_hex: &str, zcid: &str) -> Result<HashMap<String, serde_json::Value>, String> {
    let k = key32(zcid)?;
    let key16 = if k.len() >= 16 { &k[..16] } else { return Err("key32 too short".to_string()); };
    let mut raw = hex::decode(data_hex).map_err(|e| e.to_string())?;
    let iv = [0u8; 16];
    let cipher = Aes128CbcDec::new_from_slices(key16, &iv).map_err(|e| e.to_string())?;
    cipher.decrypt_padded_mut::<NoPadding>(&mut raw).map_err(|e| format!("{:?}", e))?;
    let unpadded = pkcs7_unpad(&raw)?;
    serde_json::from_slice(&unpadded).map_err(|e| e.to_string())
}
