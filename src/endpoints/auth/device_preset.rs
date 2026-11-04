use crate::core::session::device::DeviceDescriptor;
use super::AuthEndpoint;

impl AuthEndpoint {
    pub fn get_device(&self) -> DeviceDescriptor {
        DeviceDescriptor::default()
    }
}
