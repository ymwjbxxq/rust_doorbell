use aws_sdk_apigatewaymanagement::{config, Blob, Client, Endpoint};
use aws_sdk_s3::presigning::config::PresigningConfig;
use std::time::Duration;
use lambda_runtime::{handler_fn, Context, Error};
use rust_doorbell::{error::ApplicationError, dtos::send_code_dto::{SendPhotoRequest, ResponseType, PhotoResponse}, aws::client::{AWSConfig, AWSClient}};
use tracing::info;
use rust_doorbell::utils::*;

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

  let presigned_url = get_s3_presigned_url(&event, &aws_client).await?;

  let v: Vec<&str> = event.input.split('/').collect();
  send_websocket_response(&v[1].to_string(), &presigned_url).await?;

  Ok(String::from("url sent"))
}

async fn get_s3_presigned_url(event: &SendPhotoRequest, aws_client: &AWSClient) -> Result<String, ApplicationError> {
  let bucket = std::env::var("BUCKET_NAME").expect("BUCKET_NAME must be set");

  let presigned_request = aws_client.s3_client.as_ref().unwrap()
      .get_object()
      .bucket(&bucket)
      .key(&event.input)
      .presigned(PresigningConfig::expires_in(Duration::new(300, 0))?)
      .await?;

  Ok(presigned_request.uri().to_string())
}

async fn send_websocket_response(connection_id: &String, presigned_url: &String) -> Result<(), ApplicationError> {
  let domain = std::env::var("WSS_DOMAIN").expect("WSS_DOMAIN must be set");
  let endpoint = Endpoint::immutable(domain.parse().unwrap());
  let config = aws_config::load_from_env().await;
  let api_management_config = config::Builder::from(&config)
      .endpoint_resolver(endpoint)
      .build();

  let response = ResponseType::Photo(PhotoResponse {
    url: presigned_url.to_string(),
  });

  Client::from_conf(api_management_config)
      .post_to_connection()
      .connection_id(connection_id)
      .data(Blob::new(serde_json::to_string(&response).unwrap().as_bytes()))
      .send()
      .await?;

  Ok(())
}