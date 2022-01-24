use aws_sdk_apigatewaymanagement::{config, Blob, Client, Endpoint};
use lambda_runtime::{handler_fn, Context, Error};
use rust_doorbell::{error::ApplicationError, dtos::send_code_dto::{SendCodeRequest, ResponseType, SendCodeResponse}};
use tracing::info;
use rust_doorbell::utils::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
  setup_tracing();

  lambda_runtime::run(handler_fn(|event: SendCodeRequest, ctx: Context| {
    execute(event, ctx)
  }))
  .await?;
  Ok(())
}

pub async fn execute(event: SendCodeRequest, _ctx: Context) -> Result<String, ApplicationError> {
  info!("EVENT {:?}", event);
  let v: Vec<&str> = event.input.split('/').collect();
  send_websocket_response(&v[1].to_string(), &event.code).await?;

  Ok(String::from("sent"))
}


async fn send_websocket_response(connection_id: &String, code: &String) -> Result<(), ApplicationError> {
  let domain = std::env::var("WSS_DOMAIN").expect("WSS_DOMAIN must be set");
  let endpoint = Endpoint::immutable(domain.parse().unwrap());
  let config = aws_config::load_from_env().await;
  let api_management_config = config::Builder::from(&config)
      .endpoint_resolver(endpoint)
      .build();

  let response = ResponseType::Code(SendCodeResponse {
    code: code.to_string(),
  });

  Client::from_conf(api_management_config)
      .post_to_connection()
      .connection_id(connection_id)
      .data(Blob::new(serde_json::to_string(&response).unwrap().as_bytes()))
      .send()
      .await?;

  Ok(())
}