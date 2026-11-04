use rapi::core::security::aes::pkcs7_pad;
use rapi::core::security::checksum::compute_inner_checksum;
use rapi::core::security::cipher::xor_enc;
use rapi::core::security::key::key32;

#[test]
fn test_security_core() {
    let dummy_zcid = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let k = key32(dummy_zcid).expect("valid key32");
    assert_eq!(k.len(), 32);

    let data = b"hello world";
    let padded = pkcs7_pad(data, 16);
    assert_eq!(padded.len(), 16);

    let ck = compute_inner_checksum(100, 0, -1, 123456, 1, 1, 3);
    assert_ne!(ck, 0);

    let enc = xor_enc(data, 123456, None);
    assert_eq!(enc.len(), data.len());
}
