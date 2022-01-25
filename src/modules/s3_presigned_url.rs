use async_trait::async_trait;
use aws_sdk_s3::presigning::config::PresigningConfig;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use crate::error::ApplicationError;

#[async_trait]
pub trait PresignedUrl {
    async fn new(client: &aws_sdk_s3::Client) -> Self;
    async fn generate(&self, request: &PresignedUrlRequest) -> Result<String, ApplicationError>;
}

pub struct S3PresignedUrlContext;
impl S3PresignedUrlContext {
    pub async fn generate<T: PresignedUrl>(strategy: T, request: &PresignedUrlRequest) -> Result<String, ApplicationError> {
        let url = strategy.generate(request).await?;
        Ok(url)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PresignedUrlRequest {
    pub bucket_name: String,
    pub object_key: String,
    pub duration: Duration,
}

pub struct GetPresignedUrl {
    client: aws_sdk_s3::Client,
}

#[async_trait]
impl PresignedUrl for GetPresignedUrl {
    async fn new(client: &aws_sdk_s3::Client) -> Self {
        Self { 
          client: client.clone()
        }
    }

    async fn generate(&self, request: &PresignedUrlRequest) -> Result<String, ApplicationError> {
        let presigned_request = self.client
            .get_object()
            .bucket(&request.bucket_name)
            .key(&request.object_key)
            .presigned(PresigningConfig::expires_in(request.duration)?)
            .await?;

        Ok(presigned_request.uri().to_string())
    }


}

pub struct PostPresignedUrl {
    client: aws_sdk_s3::Client,
}

#[async_trait]
impl PresignedUrl for PostPresignedUrl {
    async fn new(client: &aws_sdk_s3::Client) -> Self {
        Self { 
          client: client.clone()
        }
    }

    async fn generate(&self, request: &PresignedUrlRequest) -> Result<String, ApplicationError> {
        let presigned_request = self.client
            .put_object()
            .bucket(&request.bucket_name)
            .key(&request.object_key)
            .presigned(PresigningConfig::expires_in(request.duration)?)
            .await?;

        Ok(presigned_request.uri().to_string())
    }
}
