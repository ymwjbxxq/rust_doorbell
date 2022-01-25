use lambda_runtime::{handler_fn, Context, Error};
use rust_doorbell::aws::client::{AWSClient, AWSConfig};
use rust_doorbell::dtos::s3_presigned_url_request::S3PresignedUrlRequest;
use rust_doorbell::dtos::compare_face_dto::{ResponseType, UrlResponse};
use rust_doorbell::error::ApplicationError;
use rust_doorbell::modules::s3_presigned_url::{PresignedUrlRequest, PostPresignedUrl, PresignedUrl, S3PresignedUrlContext};
use rust_doorbell::modules::web_socket::{PostWebSocketRequest, WebSocket, Socket};
use rust_doorbell::utils::*;
use std::time::Duration;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();
    // Initialize AWS client
    let config = aws_config::load_from_env().await;
    let config = AWSConfig::set_config(config);
    let aws_client = config.on_s3_presigned_url();

    lambda_runtime::run(handler_fn(|event: S3PresignedUrlRequest, ctx: Context| {
        execute(&aws_client, event, ctx)
    }))
    .await?;
    Ok(())
}

pub async fn execute(aws_client: &AWSClient, event: S3PresignedUrlRequest, _ctx: Context) -> Result<(), ApplicationError> {
    info!("EVENT {:?}", event);

    let presigned_url = generate_s3_presigned_url(&event, &aws_client).await?;
    post_to_web_socket(&event, &presigned_url).await?;

    Ok(())
}

async fn generate_s3_presigned_url(event: &S3PresignedUrlRequest, aws_client: &AWSClient) -> Result<String, ApplicationError> {
    let bucket = std::env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");
    let request = PresignedUrlRequest {
      bucket_name: bucket,
      object_key: format!(
          "guest/{connection_id}/guest.jpeg",
          connection_id = &event.detail.connection_id,
      ),
      duration: Duration::new(300, 0)
    };

    let strategy = PostPresignedUrl::new(aws_client.s3_client.as_ref().unwrap()).await;
    let url = S3PresignedUrlContext::generate(strategy, &request).await?;

    Ok(url)
}

async fn post_to_web_socket(event: &S3PresignedUrlRequest, presigned_url: &String) -> Result<(), ApplicationError> {
    let response = ResponseType::S3Url(UrlResponse {
        url: presigned_url.to_string(),
    });

    WebSocket::new()
        .await
        .post(&PostWebSocketRequest {
            connection_id: event.detail.connection_id.clone(),
            blob: response,
        })
        .await?;

    Ok(())
}
