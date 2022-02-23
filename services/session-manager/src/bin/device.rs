use aws_config::{RetryConfig, TimeoutConfig};
use lambda_http::{http::StatusCode, service_fn, Error, IntoResponse, Request, RequestExt};
use serde_json::json;
use session_manager::{
    commands::update_subscription_device::{
        UpdateSubscriptionDevice, UpdateSubscriptionDeviceCommanad,
    },
    models::device_request::DeviceRequest,
    utils::{api_helper::ApiHelper, dynamodb::AttributeValuesExt},
};
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

pub async fn execute(
    client: &aws_sdk_dynamodb::Client,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    let body = event.payload::<DeviceRequest>()?;

    if let Some(device_request) = body {
        let result = UpdateSubscriptionDevice::new()
            .await
            .execute(&client, &device_request)
            .await;

        if let Ok(result) = result {
            let attributes = result.attributes.unwrap();
            Ok(ApiHelper::response(
                StatusCode::OK,
                json!({
                    "message": format!("Remaining devices: {:?}", attributes.get_string("devices")),
                    "remaining_devices": attributes.get_number("devices")
                })
                .to_string(),
            ))
        } else {
            println!("ERROR {:?}", result.err().unwrap());
            Ok(ApiHelper::response(
                StatusCode::NOT_ACCEPTABLE,
                json!({ "message": "Limit reached, move to PRO plan" }).to_string(),
            ))
        }
    } else {
        Ok(ApiHelper::response(
            StatusCode::UNPROCESSABLE_ENTITY,
            json!({ "message": "Failed to parse JSON from request body" }).to_string(),
        ))
    }
}
