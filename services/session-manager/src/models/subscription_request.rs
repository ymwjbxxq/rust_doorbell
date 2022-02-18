use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct SubscriptionRequest {
    pub user_id: String,
    pub plan_id: String,
    pub streams: u16,
    pub devices: u16,
}
