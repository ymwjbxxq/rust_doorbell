use rust_doorbell::modules::{web_socket::{PostWebSocketRequest, Socket, WebSocket}, s3_presigned_url::{GetPresignedUrl, PresignedUrl, PresignedUrlRequest, S3PresignedUrlContext}};
use lambda_runtime::{handler_fn, Context, Error};
use rust_doorbell::{error::ApplicationError, dtos::compare_face_dto::{SendPhotoRequest, ResponseType, PhotoResponse}, aws::client::{AWSConfig, AWSClient}};
use tracing::info;
use rust_doorbell::utils::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();
    // Initialize AWS client
    let config = aws_config::load_from_env().await;
    let config = AWSConfig::set_config(config);
    let aws_client = config.on_s3_presigned_url();

    lambda_runtime::run(handler_fn(|event: SendPhotoRequest, ctx: Context| {
        execute(&aws_client, event, ctx)
    }))
    .await?;
    Ok(())
}

pub async fn execute(aws_client: &AWSClient, event: SendPhotoRequest, _ctx: Context) -> Result<String, ApplicationError> {
    info!("EVENT {:?}", event);

    let presigned_url = generate_s3_presigned_url(&event, &aws_client).await?;

    let v: Vec<&str> = event.input.split('/').collect();
    post_to_web_socket(&v[1].to_string(), &presigned_url, &aws_client).await?;

    Ok(String::from("url sent"))
}

async fn generate_s3_presigned_url(event: &SendPhotoRequest, aws_client: &AWSClient) -> Result<String, ApplicationError> {
    let bucket = std::env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");
    let request = PresignedUrlRequest {
        bucket_name: bucket,
        object_key: event.input.to_string(),
        duration: Duration::new(300, 0)
    };
    let strategy = GetPresignedUrl::new(aws_client.s3_client.as_ref().unwrap()).await;
    let url = S3PresignedUrlContext::generate(strategy, &request).await?;

    Ok(url)
}

async fn post_to_web_socket(connection_id: &String, presigned_url: &String, aws_client: &AWSClient) -> Result<(), ApplicationError> {
    let response = ResponseType::Photo(PhotoResponse {
      url: presigned_url.to_string(),
    });

    WebSocket::new(&aws_client.config)
        .await
        .post(&PostWebSocketRequest {
            connection_id: connection_id.to_string(),
            blob: response,
        })
        .await?;

    Ok(())
}