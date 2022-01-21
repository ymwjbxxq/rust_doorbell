pub fn setup_tracing() {
  let subscriber = tracing_subscriber::fmt()
    .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    .json()
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");
}