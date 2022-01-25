use lambda_runtime::{handler_fn, Context, Error};
use rand::Rng;
use rust_doorbell::error::ApplicationError;
use rust_doorbell::utils::*;
use serde_json::Value;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();

    lambda_runtime::run(handler_fn(|event: Value, ctx: Context| execute(event, ctx))).await?;
    Ok(())
}

pub async fn execute(event: Value, _ctx: Context) -> Result<String, ApplicationError> {
    info!("EVENT {:?}", event);
    Ok(format!(
        "{part1}-{part2}",
        part1 = generate_random(),
        part2 = generate_random()
    ))
}

fn generate_random() -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..999).to_string()
}
