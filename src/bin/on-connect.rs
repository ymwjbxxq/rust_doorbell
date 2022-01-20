use chrono::Duration;
use chrono::Utc;
use rust_doorbell::error::ApplicationError;
use rust_doorbell::models::connection::Connection;
use rust_doorbell::aws::client::AWSConfig;
use rust_doorbell::dtos::websocket_request::WebSocketRequest;
use lambda_runtime::{handler_fn, Context, Error};
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

  let connection = Connection {
    connection_id: event.request_context.connection_id,
    ttl_expire_at: (Utc::now() + Duration::seconds(120)).timestamp(),
  };

  add_connection(&dynamo_client, connection).await?;

  Ok(json!({
        "statusCode": 200
    }))
}

async fn add_connection(client: &aws_sdk_dynamodb::Client, connection: Connection) -> Result<(), ApplicationError> {
  let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");

  let _res = client
      .put_item()
      .table_name(&table_name)
      .set_item(Some(connection.to_dynamodb()))
      .send()
      .await?;

  Ok(())
}
