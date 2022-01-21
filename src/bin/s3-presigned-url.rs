use std::time::Duration;
use aws_sdk_apigatewaymanagement::{config, Blob, Client, Endpoint};
use uuid::Uuid;
use aws_sdk_s3::presigning::config::PresigningConfig;
use rust_doorbell::error::ApplicationError;
use lambda_runtime::{handler_fn, Context, Error};
use tracing::info;
use rust_doorbell::utils::*;
use rust_doorbell::dtos::s3_presigned_url_request::S3PresignedUrlRequest;
use rust_doorbell::aws::client::{AWSClient, AWSConfig};

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

  let presigned_url = get_s3_presigned_url(&aws_client).await?;
  send_websocket_response(event, &presigned_url).await?;

  Ok(())
}

async fn get_s3_presigned_url(aws_client: &AWSClient) -> Result<String, ApplicationError> {
  let bucket = std::env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");
  let random_key = Uuid::new_v4().to_string();
  let expires_in_1_day = Duration::new(86400, 0); 

  let presigned_request = aws_client.s3_client.as_ref().unwrap()
      .put_object()
      .bucket(&bucket)
      .key(random_key)
      .presigned(PresigningConfig::expires_in(expires_in_1_day)?)
      .await?;

  Ok(presigned_request.uri().to_string())
}

async fn send_websocket_response(event: S3PresignedUrlRequest, presigned_url: &String) -> Result<(), ApplicationError> {
  let endpoint = Endpoint::immutable(event.detail.endpoint.parse().unwrap());
  let config = aws_config::load_from_env().await;
  let api_management_config = config::Builder::from(&config)
      .endpoint_resolver(endpoint)
      .build();

  Client::from_conf(api_management_config)
      .post_to_connection()
      .connection_id(event.detail.connection_id)
      .data(Blob::new(presigned_url.as_bytes()))
      .send()
      .await?;

  Ok(())
}
