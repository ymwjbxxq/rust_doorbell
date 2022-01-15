use rust_doorbell::aws::client::AWSClient;
use rust_doorbell::dtos::request::Event;
use lambda_runtime::{handler_fn, Context, Error};
use tracing::{info};
use rust_doorbell::{utils::*};

#[tokio::main]
async fn main() -> Result<(), Error> {
  // Initialize service
  setup_tracing();
  // Initialize AWS client
  let aws_client = get_aws_client().await;

  lambda_runtime::run(handler_fn(|event: Event, ctx: Context| {
    execute(&aws_client, event, ctx)
  }))
  .await?;
  Ok(())
}

pub async fn execute(aws_client: &AWSClient, event: Event, _ctx: Context) -> Result<(), Error> {
  info!("EVENT {:?}", event);
  info!("event_type {:?}", event.request_context.event_type);
  info!("connection_id {:?}", event.request_context.connection_id);

  Ok(())
}
