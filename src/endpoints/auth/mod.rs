use std::sync::Arc;
use crate::core::base::BaseContext;

pub mod authenticate;
pub mod captcha;
pub mod device_preset;
pub mod init_zcid;
pub mod probe;
pub mod qr_login;
pub mod session_store;
pub mod two_factor;

#[derive(Clone)]
pub struct AuthEndpoint {
    pub ctx: Arc<BaseContext>,
}

impl AuthEndpoint {
    pub fn new(ctx: Arc<BaseContext>) -> Self {
        Self { ctx }
    }
}
