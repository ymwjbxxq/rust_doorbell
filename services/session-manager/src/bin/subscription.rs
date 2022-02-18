use aws_sdk_dynamodb::model::AttributeValue;
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{Value, json};
use aws_lambda_events::event::apigw::ApiGatewayProxyRequest;
use session_manager::{models::subscription_request::SubscriptionRequest};

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

    let subscriptiont_request: SubscriptionRequest = serde_json::from_str(&body).unwrap();
println!("EVENT {:?}", subscriptiont_request);
    client
        .put_item()
        .table_name(table)
        .item("pk", AttributeValue::S(subscriptiont_request.user_id.into()))
        .item("plan_id", AttributeValue::S(subscriptiont_request.plan_id.into()))
        .item("streams", AttributeValue::N(format!("{:}", subscriptiont_request.streams)))
        .item("devices", AttributeValue::N(format!("{:}", subscriptiont_request.devices)))
        .send()
        .await?;

    Ok(json!({
        "statusCode": 201
    }))
}