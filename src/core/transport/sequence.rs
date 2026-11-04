use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::protocol::codec::wrap_inner_packet;
use crate::core::protocol::frame::WireFrame;
use crate::core::security::cipher::xor_enc;

pub struct InitFrame {
    pub cmd: i32,
    pub sub: i32,
    pub bb: i32,
    pub ty: i32,
    pub pt: Vec<u8>,
}

pub fn build_init_frames(uid: i64, dest: i64, vercode: u32) -> Vec<InitFrame> {
    let vc = vercode.to_le_bytes();
    let dest_le = ((dest & 0xFFFFFFFF) as u32).to_le_bytes();
    let uid_le = ((uid & 0xFFFFFFFF) as u32).to_le_bytes();
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    let mut fr = Vec::new();
    let mut add = |cmd: i32, sub: i32, bb: i32, pt: &[u8]| {
        fr.push(InitFrame {
            cmd,
            sub,
            bb,
            ty: 1,
            pt: xor_enc(pt, uid, None),
        });
    };

    let mut buf384 = Vec::new();
    buf384.push(1);
    buf384.extend_from_slice(&vc);
    buf384.extend_from_slice(&(160u32).to_le_bytes());
    buf384.extend_from_slice(&[0u8; 8]);
    add(384, 0, 0, &buf384);

    let mut b2061 = vec![1];
    b2061.extend_from_slice(&vc);
    add(2061, 0, 0, &b2061);
    add(133, 0, 0, &[]);

    let mut buf2070 = vec![1];
    buf2070.extend_from_slice(&vc);
    buf2070.extend_from_slice(&[0u8; 4]);
    add(2070, 0, 0, &buf2070);

    let mut b1430 = vec![1];
    b1430.extend_from_slice(&vc);
    add(1430, 0, 0, &b1430);

    let mut buf151 = Vec::new();
    buf151.extend_from_slice(&uid_le);
    buf151.extend_from_slice(&[0u8; 4]);
    buf151.extend_from_slice(&(6u32).to_le_bytes());
    buf151.extend_from_slice(&[2, 4, 0, 0, 0, 0, 0]);
    add(151, 4, 0, &buf151);

    let mut b390 = vec![1];
    b390.extend_from_slice(&vc);
    add(390, 1, 0, &b390);

    let sync_str = format!(r#"{{"owner_id":"{}","data_types":[3],"src_type":3}}"#, uid);
    let mut buf1814 = Vec::new();
    buf1814.push(1);
    buf1814.extend_from_slice(&vc);
    buf1814.extend_from_slice(&(sync_str.len() as u32).to_le_bytes());
    buf1814.extend_from_slice(sync_str.as_bytes());
    add(1814, 0, 0, &buf1814);

    let mut b10301 = vec![1, 1];
    b10301.extend_from_slice(&[0u8; 4]);
    add(10301, 1, 0, &b10301);

    let mut b177 = vec![1];
    b177.extend_from_slice(&[0u8; 9]);
    add(177, 0, 0, &b177);

    let mut b525 = vec![1];
    b525.extend_from_slice(&vc);
    add(525, 0, 0, &b525);

    add(271, 0, 0, &(11u32).to_le_bytes());
    add(1715, 0, 0, &[]);

    let mut buf620 = vec![1];
    buf620.extend_from_slice(&vc);
    buf620.extend_from_slice(&[1, 0]);
    add(620, 0, 0, &buf620);

    let mut buf1582 = Vec::new();
    buf1582.push(1);
    buf1582.extend_from_slice(&vc);
    buf1582.extend_from_slice(&(160u32).to_le_bytes());
    add(1582, 0, 0, &buf1582);

    let mut buf1963 = vec![1];
    buf1963.extend_from_slice(&vc);
    buf1963.extend_from_slice(&[0u8; 11]);
    add(1963, 0, 0, &buf1963);

    add(601, 0, 0, &[0u8; 48]);
    add(1786, 0, 1, &[0u8; 8]);
    add(102, 3, 1, br#"{"seen":0,"data":[]}"#);

    let mut buf1419 = Vec::new();
    buf1419.extend_from_slice(&(6u32).to_le_bytes());
    buf1419.extend_from_slice(&vc);
    buf1419.push(1);
    add(1419, 0, 0, &buf1419);

    let mut b1411 = vec![1];
    b1411.extend_from_slice(&vc);
    add(1411, 0, 0, &b1411);

    let mut buf1484 = Vec::new();
    buf1484.push(1);
    buf1484.extend_from_slice(&vc);
    buf1484.extend_from_slice(&(1u32).to_le_bytes());
    add(1484, 0, 0, &buf1484);

    add(176, 1, 0, &[]);
    add(1407, 0, 0, &[]);

    let mut buf1399 = Vec::new();
    buf1399.extend_from_slice(&(6u32).to_le_bytes());
    buf1399.extend_from_slice(&vc);
    buf1399.push(1);
    add(1399, 0, 0, &buf1399);

    let csv = format!("1,346190,0,0,0,{},{}", now_ms - 300000, now_ms - 18000);
    let csv_bytes = csv.as_bytes();
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::new(6));
    let _ = encoder.write_all(csv_bytes);
    let gz_body = encoder.finish().unwrap_or_default();
    let mut h = crc32fast::Hasher::new();
    h.update(csv_bytes);
    let crc = h.finalize();

    let mut gz_full = Vec::new();
    gz_full.extend_from_slice(&[0x1f, 0x8b, 0x08, 0, 0, 0, 0, 0, 0, 0x03]);
    gz_full.extend_from_slice(&gz_body);
    gz_full.extend_from_slice(&crc.to_le_bytes());
    gz_full.extend_from_slice(&(csv_bytes.len() as u32).to_le_bytes());

    let mut buf1530 = Vec::new();
    buf1530.push(4);
    buf1530.extend_from_slice(&(((now_ms >> 8) & 0xFFFFFFFF) as u32).to_le_bytes());
    buf1530.extend_from_slice(&(1u32).to_le_bytes());
    buf1530.extend_from_slice(&(gz_full.len() as u32).to_le_bytes());
    buf1530.extend_from_slice(&gz_full);
    add(1530, 0, 0, &buf1530);

    let mut buf1915 = vec![1];
    buf1915.extend_from_slice(&vc);
    buf1915.extend_from_slice(&[0, 0]);
    add(1915, 0, 0, &buf1915);

    let mut buf396 = Vec::new();
    buf396.push(1);
    buf396.extend_from_slice(&vc);
    buf396.extend_from_slice(&(1u32).to_le_bytes());
    buf396.extend_from_slice(&(100u32).to_le_bytes());
    buf396.extend_from_slice(&(160u32).to_le_bytes());
    buf396.extend_from_slice(&[0u8; 4]);
    buf396.push(1);
    add(396, 4, 0, &buf396);

    let mut buf376 = Vec::new();
    buf376.push(1);
    buf376.extend_from_slice(&vc);
    buf376.extend_from_slice(&[0u8; 4]);
    buf376.extend_from_slice(&(100u32).to_le_bytes());
    buf376.extend_from_slice(&(160u32).to_le_bytes());
    add(376, 1, 0, &buf376);

    let mut buf272 = Vec::new();
    buf272.push(1);
    buf272.extend_from_slice(&vc);
    buf272.extend_from_slice(&[1, 0, 0x19]);
    buf272.extend_from_slice(&[0u8; 7]);
    add(272, 1, 0, &buf272);

    let mut buf2085 = vec![1];
    buf2085.extend_from_slice(&vc);
    buf2085.extend_from_slice(&[0u8; 6]);
    buf2085.push(1);
    buf2085.extend_from_slice(&[0u8; 7]);
    add(2085, 0, 0, &buf2085);

    let mut buf700 = vec![1];
    buf700.extend_from_slice(&vc);
    buf700.push(0);
    add(700, 0, 0, &buf700);

    let mut buf1588 = Vec::new();
    buf1588.push(1);
    buf1588.extend_from_slice(&vc);
    buf1588.extend_from_slice(&(160u32).to_le_bytes());
    buf1588.push(0);
    add(1588, 1, 0, &buf1588);

    let mut b2190 = vec![1];
    b2190.extend_from_slice(&vc);
    add(2190, 0, 0, &b2190);

    let e2ee_json = format!(
        r#"{{"devices":{{"masterId":"aade588b835d3813157f91c24d3b5394","encIdentity":"BSZhp3pIB9XiGN6DxHr\/Cf\/egcZwhiXk67b\/tqBgSFB","lastUpdateTs":{},"encSignature":"0idIG9lZ8c444kPv\/axpIi5KiaMQ2Lcb\/54Zs+kwphSHTr9hV3H\/StaVL4klnyNtsEwtDiyDVlqmPBVWICBg==","companions":[]}}}}"#,
        now_ms
    );
    let mut buf3400 = Vec::new();
    buf3400.push(1);
    buf3400.extend_from_slice(&vc);
    buf3400.extend_from_slice(&[1, 1, 0x15, 1, 0, 0]);
    buf3400.extend_from_slice(e2ee_json.as_bytes());
    add(3400, 0, 0, &buf3400);

    let mut b1971 = vec![1];
    b1971.extend_from_slice(&dest_le);
    add(1971, 0, 0, &b1971);

    let mut buf1708 = Vec::new();
    buf1708.push(1);
    buf1708.extend_from_slice(&vc);
    buf1708.extend_from_slice(&[0u8; 6]);
    buf1708.extend_from_slice(&dest_le);
    buf1708.extend_from_slice(&[0u8; 8]);
    add(1708, 0, 0, &buf1708);

    let mut buf151_dest = Vec::new();
    buf151_dest.extend_from_slice(&dest_le);
    buf151_dest.extend_from_slice(&[0u8; 4]);
    buf151_dest.extend_from_slice(&(6u32).to_le_bytes());
    buf151_dest.extend_from_slice(&[4, 4, 0, 0, 0, 0, 0]);
    add(151, 4, 0, &buf151_dest);

    let mut buf1251 = Vec::new();
    buf1251.extend_from_slice(&[0, 0, 1]);
    buf1251.extend_from_slice(&vc);
    buf1251.extend_from_slice(&[0, 0]);
    buf1251.extend_from_slice(&dest_le);
    buf1251.extend_from_slice(&(6u32).to_le_bytes());
    buf1251.extend_from_slice(&(240u32).to_le_bytes());
    buf1251.extend_from_slice(&[0u8; 3]);
    add(1251, 4, 0, &buf1251);

    let mut buf1400 = vec![5, 0];
    buf1400.extend_from_slice(&[0xFF; 13]);
    add(1400, 4, 0, &buf1400);

    fr
}

pub fn send_init_frames(
    stream: &mut TcpStream,
    dk: &[u8],
    uid: i64,
    lock: Option<&Mutex<()>>,
) -> Result<(), String> {
    let frames = build_init_frames(uid, 335694418, 260802903);
    let mut seq = -1i32;
    for item in frames {
        seq -= 1;
        let inner = wrap_inner_packet(item.cmd, item.sub, uid, &item.pt, seq, item.bb, item.ty, 3, None);
        let outer = WireFrame::build(&inner, dk)?;
        if let Some(l) = lock {
            let _guard = l.lock().unwrap();
            let _ = stream.write_all(&outer);
        } else {
            let _ = stream.write_all(&outer);
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    Ok(())
}
