use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct DeviceRequest {
    pub user_id: String,
    pub devices_count: u16,
}
