use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct StreamRequest {
    pub user_id: String,
    pub video_id: String,
}
