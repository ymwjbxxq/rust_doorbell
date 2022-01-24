use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct S3PresignedUrlRequest {
  pub detail: DetailMessage,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DetailMessage {
  pub connection_id: String,
}
