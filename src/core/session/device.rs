use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceDescriptor {
    pub model: String,
    pub os_name: String,
    pub os_version: String,
    pub hardware: String,
    pub imei: String,
}

impl Default for DeviceDescriptor {
    fn default() -> Self {
        Self {
            model: "samsung_SM-J610F_11".to_string(),
            os_name: "android".to_string(),
            os_version: "11".to_string(),
            hardware: "qcom".to_string(),
            imei: "000000000000000".to_string(),
        }
    }
}
