use serde::de::{self, Deserializer};
use serde::Deserialize;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Deserialize, Debug, Clone)]
pub struct SubscriptionRequest {
    pub user_id: String,
    pub plan_id: String,

    // #[serde(deserialize_with = "from_str")]
    pub streams: u16,

    // #[serde(deserialize_with = "from_str")]
    pub devices: u16,
}

// fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
// where
//     T: FromStr,
//     T::Err: Display,
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     T::from_str(&s).map_err(de::Error::custom)
// }
