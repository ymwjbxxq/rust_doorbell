use aws_sdk_dynamodb::{model::{AttributeValue, ReturnValue}};
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{Value, json};
use aws_lambda_events::event::apigw::ApiGatewayProxyRequest;
use session_manager::{models::device_request::DeviceRequest};

#[tokio::main]
async fn main() -> Result<(), Error> {
  let config = aws_config::load_from_env().await;
    let dynamodb_client = aws_sdk_dynamodb::Client::new(&config);

    lambda_runtime::run(handler_fn(|event: ApiGatewayProxyRequest, ctx: Context| {
        execute(&dynamodb_client, event, ctx)
    }))
    .await?;
    Ok(())
}

pub async fn execute(client: &aws_sdk_dynamodb::Client, event: ApiGatewayProxyRequest, _ctx: Context) -> Result<Value, Error> {
    let table = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let body = event.body.expect("body must be set");

    let device_request: DeviceRequest = serde_json::from_str(&body).unwrap();

    let result = client
        .update_item()
        .table_name(table)
        .key("pk", AttributeValue::S(device_request.user_id.into()))
        .update_expression("set devices = devices - :incr")
        .expression_attribute_values(":incr", AttributeValue::N(format!("{:}", device_request.devices_count)))
         .expression_attribute_values(":limit", AttributeValue::N(format!("{:}", 0)))
        .condition_expression("attribute_exists(pk) AND devices > :limit")
        .return_values(ReturnValue::UpdatedNew)
        .send()
        .await?;

    println!("{:?}", result);

    Ok(json!({
        "statusCode": 200
    }))
}