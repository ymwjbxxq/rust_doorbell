use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseType {
    Error(String),
    Code(SendCodeResponse),
    Audio(AudioResponse),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SendCodeRequest {
    pub input: String,
    pub bucket_name: String,
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SendCodeResponse {
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AudioResponse {
    pub input: String,
    pub bucket_name: String,
}