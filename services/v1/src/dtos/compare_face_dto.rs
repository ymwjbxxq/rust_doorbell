use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ResponseType {
    Error(String),
    Code(SendCodeResponse),
    Audio(AudioResponse),
    Photo(PhotoResponse),
    S3Url(UrlResponse),
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct UrlResponse {
    pub url: String,
}