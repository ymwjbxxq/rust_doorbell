use rust_doorbell::aws::client::AWSClient;
use aws_sdk_dynamodb::model::AttributeValue;
use rust_doorbell::aws::client::{AWSConfig};
use lambda_runtime::{handler_fn, Context, Error};
use rust_doorbell::error::ApplicationError;
use tracing::{info};
use rust_doorbell::{utils::*};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
  setup_tracing();
  // Initialize AWS client
  let config = aws_config::load_from_env().await;
  let config = AWSConfig::set_config(config);
  let aws_client = config.on_disconnect();

  lambda_runtime::run(handler_fn(|event: Value, ctx: Context| {
    execute(&aws_client, event, ctx)
  }))
  .await?;
  Ok(())
}

pub async fn execute(aws_client: &AWSClient, event: Value, _ctx: Context) -> Result<String, ApplicationError> {
  info!("EVENT {:?}", event);

  Ok(String::from("sent"))
}