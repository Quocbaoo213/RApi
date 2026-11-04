use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::core::contract::event::MessageEvent;
use crate::core::protocol::codec::{parse_incoming_frame, InboundPacket};
use crate::core::protocol::frame::WireFrame;
use crate::core::protocol::header::PacketHeader;
use crate::core::protocol::opcode::{CMD_PING, SUB_PING};
use crate::core::security::checksum::{compute_checksum, MAGIC_CHECKSUM_XOR};
use crate::core::security::key::parse_dk;
use crate::core::session::credential::SessionProfile;
use super::dispatcher::PacketDispatcher;
use super::handshake::build_prov_frame;
use super::heartbeat::HeartbeatManager;
use super::sequence::send_init_frames;

pub struct SocketConnection {
    pub uid: i64,
    pub dk: Vec<u8>,
    pub session_key: String,
    pub ksid: String,
    pub server_pub_key_b64: String,
    pub servers: Vec<HashMap<String, serde_json::Value>>,

    stream: Arc<Mutex<Option<TcpStream>>>,
    write_lock: Arc<Mutex<()>>,
    is_connected: Arc<AtomicBool>,
    is_running: Arc<AtomicBool>,

    seq_counter: Arc<AtomicI32>,
    cmsg_counter: Arc<AtomicI32>,
    ck_counter: Arc<AtomicU32>,

    pub dispatcher: PacketDispatcher,
    heartbeat: HeartbeatManager,
}

impl SocketConnection {
    pub fn new(profile: &SessionProfile) -> Result<Arc<Self>, String> {
        let dk = parse_dk(&profile.dk_hex)?;
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        let cmsg = (now_ms & 0x7FFFFFFF) as i32;
        let ck = ((now_ms ^ (MAGIC_CHECKSUM_XOR as i64)) & 0xFFFFFFFF) as u32;

        let conn = Arc::new(Self {
            uid: profile.uid,
            dk,
            session_key: profile.session_key.clone(),
            ksid: profile.ksid.clone(),
            server_pub_key_b64: profile.ss_pub_key.clone(),
            servers: profile.socket_servers.clone(),
            stream: Arc::new(Mutex::new(None)),
            write_lock: Arc::new(Mutex::new(())),
            is_connected: Arc::new(AtomicBool::new(false)),
            is_running: Arc::new(AtomicBool::new(false)),
            seq_counter: Arc::new(AtomicI32::new(-50)),
            cmsg_counter: Arc::new(AtomicI32::new(cmsg)),
            ck_counter: Arc::new(AtomicU32::new(ck)),
            dispatcher: PacketDispatcher::new(),
            heartbeat: HeartbeatManager::new(),
        });

        Ok(conn)
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected.load(Ordering::SeqCst)
    }

    pub fn get_next_counters(&self) -> (i32, i32, u32) {
        let seq = self.seq_counter.fetch_sub(1, Ordering::SeqCst) - 1;
        let cmsg = self.cmsg_counter.fetch_add(1, Ordering::SeqCst) + 1;
        let ck = self.ck_counter.fetch_add(1, Ordering::SeqCst) + 1;
        (seq, cmsg, ck)
    }

    pub fn connect(self: &Arc<Self>) -> bool {
        if self.is_connected() {
            return true;
        }

        let mut pools = self.servers.clone();
        if pools.is_empty() {
            let mut def = HashMap::new();
            def.insert(
                "host".to_string(),
                serde_json::Value::String("49.213.95.79".to_string()),
            );
            def.insert("port".to_string(), serde_json::Value::from(443));
            def.insert(
                "pubKey".to_string(),
                serde_json::Value::String(
                    "ijmSP7FPSQyy7HRTeNQa8/JlLmC4wJ4rXiQydeOze3o=".to_string(),
                ),
            );
            pools.push(def);
        }

        for srv in pools {
            let host = srv
                .get("host")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if host.is_empty() {
                continue;
            }
            let port = srv
                .get("port")
                .and_then(|v| v.as_i64())
                .unwrap_or(443) as u16;
            let pub_key = srv
                .get("pubKey")
                .or_else(|| srv.get("pubkey"))
                .and_then(|v| v.as_str())
                .unwrap_or(&self.server_pub_key_b64)
                .to_string();

            let addr = format!("{}:{}", host, port);
            let mut tcp_stream = match TcpStream::connect_timeout(
                &addr.parse().unwrap_or_else(|_| "49.213.95.79:443".parse().unwrap()),
                Duration::from_secs(10),
            ) {
                Ok(s) => s,
                Err(_) => continue,
            };

            let _ = tcp_stream.set_nodelay(true);
            let frame0 = match build_prov_frame(
                &self.session_key,
                &self.dk,
                &self.ksid,
                &pub_key,
                self.uid,
            ) {
                Ok(f) => f,
                Err(_) => continue,
            };

            let http_header = format!(
                "GET/HTTP/1.1\r\nHost: {}\r\nUser-Agent: Mozilla/5.0\r\n\r\n",
                host
            );
            if tcp_stream.write_all(http_header.as_bytes()).is_err() {
                continue;
            }
            thread::sleep(Duration::from_millis(50));

            let _ = tcp_stream.set_read_timeout(Some(Duration::from_secs(2)));
            let mut hdr_buf = [0u8; 1024];
            let _ = tcp_stream.read(&mut hdr_buf);

            if tcp_stream.write_all(&frame0).is_err() {
                continue;
            }

            let _ = tcp_stream.set_read_timeout(Some(Duration::from_secs(5)));
            let mut resp_hdr = [0u8; 5];
            if tcp_stream.read_exact(&mut resp_hdr).is_err() {
                continue;
            }
            let total_len = u32::from_le_bytes([
                resp_hdr[0],
                resp_hdr[1],
                resp_hdr[2],
                resp_hdr[3],
            ]) as usize;
            if total_len < 5 {
                continue;
            }
            let mut body = vec![0u8; total_len - 5];
            if tcp_stream.read_exact(&mut body).is_err() {
                continue;
            }
            let _ = tcp_stream.set_read_timeout(None);

            let f_type = resp_hdr[4];
            if matches!(f_type, 1 | 2 | 3 | 4 | 8) {
                let _ = send_init_frames(
                    &mut tcp_stream,
                    &self.dk,
                    self.uid,
                    Some(&self.write_lock),
                );

                *self.stream.lock().unwrap() = Some(tcp_stream);
                self.is_connected.store(true, Ordering::SeqCst);
                self.is_running.store(true, Ordering::SeqCst);

                let conn_clone = self.clone();
                thread::spawn(move || {
                    conn_clone.recv_loop();
                });

                self.heartbeat.start(self.clone(), Duration::from_secs(10));
                return true;
            }
        }

        false
    }

    fn recv_loop(&self) {
        let mut buf = Vec::with_capacity(65536);
        let mut temp = [0u8; 32768];

        while self.is_running.load(Ordering::SeqCst) && self.is_connected.load(Ordering::SeqCst) {
            let mut stream_guard = self.stream.lock().unwrap();
            let n = if let Some(ref mut stream) = *stream_guard {
                let _ = stream.set_read_timeout(Some(Duration::from_secs(15)));
                match stream.read(&mut temp) {
                    Ok(0) => break,
                    Ok(bytes) => bytes,
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => continue,
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                    Err(_) => break,
                }
            } else {
                break;
            };
            drop(stream_guard);

            buf.extend_from_slice(&temp[..n]);

            while buf.len() >= 5 {
                let total_len = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
                if total_len < 5 {
                    buf.clear();
                    break;
                }
                if buf.len() < total_len {
                    break;
                }

                let frame_bytes: Vec<u8> = buf.drain(..total_len).collect();
                let frame = WireFrame {
                    total_len: total_len as u32,
                    frame_type: frame_bytes[4],
                    body: frame_bytes[5..].to_vec(),
                };

                if let Ok(Some(parsed)) = parse_incoming_frame(&frame, &self.dk, self.uid) {
                    self.dispatcher.dispatch(parsed);
                }
            }
        }

        self.close();
    }

    pub fn send_packet(
        &self,
        inner_packet: &[u8],
        wait_ack: bool,
        cmd: u16,
        timeout: Duration,
    ) -> Result<Option<InboundPacket>, String> {
        if !self.is_connected() {
            return Err("socket not connected".to_string());
        }

        let outer = WireFrame::build(inner_packet, &self.dk)?;
        let rx = if wait_ack && cmd > 0 {
            Some(self.dispatcher.prepare_cmd(cmd))
        } else {
            None
        };

        {
            let _guard = self.write_lock.lock().unwrap();
            let mut stream_guard = self.stream.lock().unwrap();
            if let Some(ref mut s) = *stream_guard {
                s.write_all(&outer).map_err(|e| e.to_string())?;
            } else {
                return Err("stream is None".to_string());
            }
        }

        if let Some(receiver) = rx {
            let to = if timeout.is_zero() {
                Duration::from_secs(4)
            } else {
                timeout
            };
            let start = Instant::now();
            while start.elapsed() < to {
                if let Ok(pkt) = receiver.recv_timeout(Duration::from_millis(50)) {
                    self.dispatcher.remove_cmd(cmd);
                    return Ok(Some(pkt));
                }
            }
            self.dispatcher.remove_cmd(cmd);
            return Err("timeout waiting for ack".to_string());
        }

        Ok(None)
    }

    pub fn send_ping(&self) -> Result<(), String> {
        let (seq, _, _) = self.get_next_counters();
        let ck = compute_checksum(
            CMD_PING as i32,
            SUB_PING as i32,
            seq,
            self.uid,
            0,
            0,
            0,
            1,
            2,
            3,
        );
        let hdr = PacketHeader {
            ck,
            bb: 1,
            ty: 2,
            seq,
            uid: (self.uid & 0xFFFFFFFF) as u32,
            ver: 3,
            cmd: CMD_PING,
            sub: SUB_PING,
        };
        let _ = self.send_packet(&hdr.pack(), false, 0, Duration::ZERO);
        Ok(())
    }

    pub fn on_message<F>(&self, handler: F)
    where
        F: Fn(MessageEvent) + Send + Sync + 'static,
    {
        self.dispatcher.set_message_handler(handler);
    }

    pub fn close(&self) {
        self.is_running.store(false, Ordering::SeqCst);
        if self.is_connected.swap(false, Ordering::SeqCst) {
            self.heartbeat.stop();
            let mut stream_guard = self.stream.lock().unwrap();
            if let Some(stream) = stream_guard.take() {
                let _ = stream.shutdown(std::net::Shutdown::Both);
            }
        }
    }
}
