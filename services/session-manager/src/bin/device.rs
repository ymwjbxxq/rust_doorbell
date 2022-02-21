use aws_sdk_dynamodb::model::{AttributeValue, ReturnValue};
use lambda_http::{http::StatusCode, service_fn, Body, Error, IntoResponse, Request, Response};
use serde_json::json;
use session_manager::{models::device_request::DeviceRequest, utils::dynamodb::AttributeValuesExt};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    lambda_http::run(service_fn(|event: Request| {
        execute(&dynamodb_client, event)
    }))
    .await?;
    Ok(())
}

pub async fn execute(client: &aws_sdk_dynamodb::Client, event: Request) -> Result<impl IntoResponse, Error> {
    let table = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let body: Result<DeviceRequest, serde_json::Error> = match event.body() {
        Body::Text(body) => serde_json::from_str(body),
        _ => {
            return Ok(response(
                StatusCode::BAD_REQUEST,
                json!({ "message": "Empty request body" }).to_string(),
            ));
        }
    };

    if let Ok(device_request) = body {
        let result = client
            .update_item()
            .table_name(table)
            .key("pk", AttributeValue::S(device_request.user_id.into()))
            .update_expression("set devices = devices - :incr")
            .expression_attribute_values(
                ":incr",
                AttributeValue::N(format!("{:}", device_request.devices_count)),
            )
            .expression_attribute_values(":limit", AttributeValue::N(format!("{:}", 0)))
            .condition_expression("attribute_exists(pk) AND devices > :limit")
            .return_values(ReturnValue::UpdatedNew)
            .send()
            .await;

        Ok(match result {
            Ok(result) => {
                let attributes = result.attributes.unwrap();
                response(
                    StatusCode::OK,
                json!({ 
                  "message": format!("Remaining devices: {:?}", attributes.get_string("devices")), 
                  "remaining_devices": attributes.get_number("devices") 
                }).to_string())
            }
            _ => response(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "message": "Limit reached, move to PRO plan" }).to_string(),
            ),
        })
    } else {
        Ok(response(
            StatusCode::BAD_REQUEST,
            json!({ "message": "Failed to parse device from request body" }).to_string(),
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
