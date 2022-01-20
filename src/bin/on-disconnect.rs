use aws_sdk_dynamodb::model::AttributeValue;
use rust_doorbell::aws::client::{AWSConfig};
use rust_doorbell::dtos::websocket_request::WebSocketRequest;
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
  let dynamo_client = config.dynamo_client();

  lambda_runtime::run(handler_fn(|event: WebSocketRequest, ctx: Context| {
    execute(&dynamo_client, event, ctx)
  }))
  .await?;
  Ok(())
}

pub async fn execute(dynamo_client: &aws_sdk_dynamodb::Client, event: WebSocketRequest, _ctx: Context) -> Result<Value, ApplicationError> {
  info!("EVENT {:?}", event);

  delete_connection(&dynamo_client, &event.request_context.connection_id).await?;

  Ok(json!({
        "statusCode": 200
    }))
}

async fn delete_connection(client: &aws_sdk_dynamodb::Client, connection_id: &str) -> Result<(), ApplicationError> {
  let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");

  let _res = client
      .delete_item()
      .table_name(&table_name)
      .key("connection_id", AttributeValue::S(connection_id.to_string()))
      .send()
      .await?;

  Ok(())
}
