use aws_config::{RetryConfig, TimeoutConfig};
use lambda_http::RequestExt;
use lambda_http::{http::StatusCode, service_fn, IntoResponse, Request};
use serde_json::json;
use session_manager::commands::add_subscription::{AddSubscription, AddSubscriptionCommanad};
use session_manager::utils::api_helper::ApiHelper;
use std::time::Duration;

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), E> {
    let config = aws_config::from_env()
        .retry_config(RetryConfig::new().with_max_attempts(10))
        .timeout_config(
            TimeoutConfig::new()
                .with_read_timeout(Some(Duration::from_secs(1)))
                .with_connect_timeout(Some(Duration::from_secs(1)))
                .with_api_call_timeout(Some(Duration::from_secs(1))),
        )
        .load()
        .await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    lambda_http::run(service_fn(|event: Request| {
        execute(&dynamodb_client, event)
    }))
    .await?;
    Ok(())
}

pub async fn execute(
    client: &aws_sdk_dynamodb::Client,
    event: Request,
) -> Result<impl IntoResponse, E> {
    let body = event.payload()?;
    if let Some(request) = body {
        let result = AddSubscription::new()
            .await
            .execute(&client, &request)
            .await;
        if result.is_ok() {
            Ok(ApiHelper::response(
                StatusCode::CREATED,
                json!({"message": "Subscription added"}).to_string(),
            ))
        } else {
            println!("ERROR {:?}", result.err().unwrap());
            Ok(ApiHelper::response(
                StatusCode::NOT_ACCEPTABLE,
                json!({ "message": "Subscription already existing" }).to_string(),
            ))
        }
    } else {
        Ok(ApiHelper::response(
            StatusCode::UNPROCESSABLE_ENTITY,
            json!({"message":"Failed to parse JSON from request body"}).to_string(),
        ))
    }
}
