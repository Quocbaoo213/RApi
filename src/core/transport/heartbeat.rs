use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use super::connection::SocketConnection;

pub struct HeartbeatManager {
    is_running: Arc<AtomicBool>,
}

impl HeartbeatManager {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self, conn: Arc<SocketConnection>, interval: Duration) {
        self.is_running.store(true, Ordering::SeqCst);
        let flag = self.is_running.clone();
        thread::spawn(move || {
            while flag.load(Ordering::SeqCst) {
                thread::sleep(interval);
                if !flag.load(Ordering::SeqCst) {
                    break;
                }
                if !conn.is_connected() {
                    break;
                }
                let _ = conn.send_ping();
            }
        });
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
}
