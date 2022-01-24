use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseType {
    Error(String),
    Code(SendCodeResponse),
    Audio(AudioResponse),
    Photo(PhotoResponse),
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SendCodeRequest {
    pub input: String,
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SendPhotoRequest {
    pub input: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct SendCodeResponse {
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct AudioResponse {
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PhotoResponse {
    pub url: String,
}