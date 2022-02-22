use std::time::Duration;
use aws_config::{TimeoutConfig, RetryConfig};
use lambda_http::{http::StatusCode, service_fn, Error, IntoResponse, Request, RequestExt};
use serde_json::json;
use session_manager::{models::device_request::DeviceRequest, 
  utils::{dynamodb::AttributeValuesExt, api_helper::ApiHelper}, 
  commands::update_subscription_device::{UpdateSubscriptionDevice, UpdateSubscriptionDeviceCommanad}}
;

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
    let body = event.payload::<DeviceRequest>()?;

    if let Some(device_request) = body {
        let result = UpdateSubscriptionDevice::new().await
            .execute(&client, &device_request)
            .await;

        Ok(match result {
            Ok(result) => {
                let attributes = result.attributes.unwrap();
                ApiHelper::response(
                    StatusCode::OK,
                json!({ 
                    "message": format!("Remaining devices: {:?}", attributes.get_string("devices")), 
                    "remaining_devices": attributes.get_number("devices") 
                }).to_string())
            }
            _ => {
              println!("ERROR {:?}", result.err().unwrap());
              ApiHelper::response(
                  StatusCode::INTERNAL_SERVER_ERROR,
                  json!({ "message": "Limit reached, move to PRO plan" }).to_string(),
              )
            },
        })
    } else {
        Ok(ApiHelper::response(
            StatusCode::BAD_REQUEST,
            json!({ "message": "Failed to parse device from request body" }).to_string(),
        ))
    }
}