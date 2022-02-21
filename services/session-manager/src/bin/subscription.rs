use aws_config::{RetryConfig, TimeoutConfig};
use aws_sdk_dynamodb::model::AttributeValue;
use futures::{try_join, TryFutureExt};
use lambda_http::RequestExt;
use lambda_http::{http::StatusCode, service_fn, IntoResponse, Request, Response};
use serde_json::json;
use session_manager::error::ApplicationError;
use session_manager::models::subscription_request::SubscriptionRequest;
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

pub async fn execute(client: &aws_sdk_dynamodb::Client, event: Request) -> Result<impl IntoResponse,E> {
    let body = event.payload()?;
    if let Some(request) = body {
//         let devices = device(&client, &request)
//             .map_err(|_e| ApplicationError::InitError("Could not setup devices".to_string()));
//         let streams = streaming(&client, &request)
//             .map_err(|_e| ApplicationError::InitError("Could not setup streaming".to_string()));
//         let result = try_join!(devices, streams);
// println!("try_join {:?}", result);

        let result = device(&client, &request).await;
        if result.is_ok() {
            Ok(response(
                StatusCode::OK,
                json!({"message": "Subscription inserted"}).to_string(),
            ))
        } else {
            Ok(response(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"message":format!("{:?}", result.err().unwrap())}).to_string(),
            ))
        }
    } else {
        Ok(response(
            StatusCode::BAD_REQUEST,
            json!({"message":"Failed to parse subscription from request body"}).to_string(),
        ))
    }
}

async fn device(client: &aws_sdk_dynamodb::Client, request: &SubscriptionRequest) -> Result<(), ApplicationError> {
    let table = std::env::var("SUBSCRIPTION_TABLE_NAME").expect("SUBSCRIPTION_TABLE_NAME must be set");
    client
        .put_item()
        .table_name(table)
        .item("pk", AttributeValue::S(request.user_id.to_string()))
        .item("plan_id", AttributeValue::S(request.plan_id.to_string()))
        .item(
            "devices",
            AttributeValue::N(format!("{:}", request.devices)),
        )
        .item(
            "streams",
            AttributeValue::N(format!("{:}", request.streams)),
        )
        .send()
        .await?;

    Ok(())
}

// pub async fn streaming(client: &aws_sdk_dynamodb::Client, request: &SubscriptionRequest) -> Result<(), ApplicationError> {
//     let table = std::env::var("STREAMING_TABLE_NAME").expect("STREAMING_TABLE_NAME must be set");
//     client
//         .put_item()
//         .table_name(table)
//         .item("pk", AttributeValue::S(request.user_id.to_string()))
//         .item("plan_id", AttributeValue::S(request.plan_id.to_string()))
//         .item(
//             "streams",
//             AttributeValue::N(format!("{:}", request.streams)),
//         )
//         .send()
//         .await?;

//     Ok(())
// }

fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}
