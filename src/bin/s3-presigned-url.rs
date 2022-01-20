
use rust_doorbell::error::ApplicationError;
use lambda_runtime::{handler_fn, Context, Error};
use tracing::{info};
use rust_doorbell::{utils::*};
use serde_json::{Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
  setup_tracing();
  // Initialize AWS client

  lambda_runtime::run(handler_fn(|event: Value, ctx: Context| {
    execute(event, ctx)
  }))
  .await?;
  Ok(())
}

pub async fn execute(event: Value, _ctx: Context) -> Result<(), ApplicationError> {
  info!("EVENT {:?}", event);


  Ok(())
}
