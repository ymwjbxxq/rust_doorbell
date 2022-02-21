use aws_config::{RetryConfig, TimeoutConfig};
use aws_sdk_dynamodb::model::AttributeValue;
use lambda_http::{http::StatusCode, service_fn, Body, Error, IntoResponse, Request, Response};
use serde_json::json;
use session_manager::models::subscription_request::SubscriptionRequest;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
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

pub async fn execute(client: &aws_sdk_dynamodb::Client, event: Request) -> Result<impl IntoResponse, Error> {
    let table = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let body: Result<SubscriptionRequest, serde_json::Error> = match event.body() {
        Body::Text(body) => serde_json::from_str(body),
        _ => {
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({"message":"Empty request body"}).to_string(),
            ));
        }
    };

    if let Ok(subscriptiont_request) = body {
        let result = client
            .put_item()
            .table_name(table)
            .item(
                "pk",
                AttributeValue::S(subscriptiont_request.user_id.into()),
            )
            .item(
                "plan_id",
                AttributeValue::S(subscriptiont_request.plan_id.into()),
            )
            .item(
                "streams",
                AttributeValue::N(format!("{:}", subscriptiont_request.streams)),
            )
            .item(
                "devices",
                AttributeValue::N(format!("{:}", subscriptiont_request.devices)),
            )
            .send()
            .await;

        Ok(match result {
            Ok(_result) => response(
                StatusCode::OK,
                json!({"message":"Subscription added"}).to_string(),
            ),
            _ => response(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"message":"Failed to add subscription"}).to_string(),
            ),
        })
    } else {
        Ok(response(
            StatusCode::BAD_REQUEST,
            json!({"message":"Failed to parse subscription from request body"}).to_string(),
        ))
    }
}

fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}
