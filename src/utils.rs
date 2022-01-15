use crate::aws::client::AWSClient;
use crate::aws::client::AWSConfig;
use tracing::{info, instrument};

/// Setup tracing
pub fn setup_tracing() {
  let subscriber = tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .json()
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");
}

/// Initialize AWS client
#[instrument]
pub async fn get_aws_client() -> AWSClient {
  let config = aws_config::load_from_env().await;
  let config = AWSConfig::set_config(config);
  let aws_client = config.init();

  return aws_client;
}