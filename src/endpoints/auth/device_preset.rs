pub const REG_URL: &str = "https://register.zaloapp.com";
pub const ACC_URL: &str = "https://accounts.zalo.me";
pub const ZMVC_URL: &str = "https://zm-verification-center.zaloapp.com";
pub const ZMCAP_URL: &str = "https://zmcaptcha.zaloapp.com";

pub const URL_PHONE_VERIFY: &str = "https://register.zaloapp.com/api/v1/login/phone/verify";
pub const URL_ACTIVE_BY_PASSWORD: &str = "https://register.zaloapp.com/api/register/activeAccountByPassword";
pub const URL_CHECK_SESSION: &str = "https://accounts.zalo.me/verify/v3/api/check-session";
pub const URL_GET_QUESTION: &str = "https://accounts.zalo.me/verify/v3/api/seq/get-question";
pub const URL_ANSWER_QUESTION: &str = "https://accounts.zalo.me/verify/v3/api/seq/answer";
pub const URL_CAPTCHA_VERIFY: &str = "https://zmcaptcha.zaloapp.com/api/slide-captcha-verify";

use crate::core::session::device::DeviceDescriptor;
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn get_device(&self) -> DeviceDescriptor {
        DeviceDescriptor::default()
    }
}
