use aws_config::{RetryConfig, TimeoutConfig};
use aws_sdk_dynamodb::model::{AttributeValue, ReturnValue};
use lambda_http::{
    http::StatusCode, service_fn, Error, IntoResponse, Request, RequestExt, Response,
};
use serde_json::json;
use session_manager::{
    error::ApplicationError, models::stream_request::StreamRequest,
    utils::dynamodb::AttributeValuesExt,
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
    let body = event.payload::<StreamRequest>()?;

    if let Some(request) = body {
        let subscription = get_subscription(client, &request).await?;
        if let Some(subscription) = subscription.item {
            let allowed_streams = subscription.get_number("streams").unwrap();
            let streams = get_streams(client, &request).await?;
            let current_streams = streams.count();

            let found = streams.items.unwrap().iter().any(|row| {
                row.get_string("pk").unwrap() == request.user_id
                    && row.get_string("sk").unwrap() == request.video_id
            });

            if found {
                update_ttl(client, &request).await?;
                return Ok(response(
                    StatusCode::OK,
                    json!({ "message": "ttl updated" }).to_string(),
                ));
            }

            if allowed_streams == current_streams {
                return Ok(response(
                    StatusCode::NOT_ACCEPTABLE,
                    json!({ "message": "Too many active streams" }).to_string(),
                ));
            }
            add_stream(client, &request).await?;
            return Ok(response(
                StatusCode::CREATED,
                json!({ "message": "Stream added" }).to_string(),
            ));
        }
        return Ok(response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "message": "No subscription found" }).to_string(),
        ));
    }

    return Ok(response(
        StatusCode::BAD_REQUEST,
        json!({ "message": "Failed to parse streams from request body" }).to_string(),
    ));
}

async fn get_streams(
    client: &aws_sdk_dynamodb::Client,
    request: &StreamRequest,
) -> Result<aws_sdk_dynamodb::output::QueryOutput, ApplicationError> {
    let streaming_table =
        std::env::var("STREAMING_TABLE_NAME").expect("STREAMING_TABLE_NAME must be set");
    let streams = client
        .query()
        .table_name(&streaming_table)
        .key_condition_expression("pk = :pk ")
        .expression_attribute_values(":pk", AttributeValue::S(request.user_id.to_string()))
        .send()
        .await?;
    Ok(streams)
}

async fn get_subscription(
    client: &aws_sdk_dynamodb::Client,
    request: &StreamRequest,
) -> Result<aws_sdk_dynamodb::output::GetItemOutput, ApplicationError> {
    let subscription_table =
        std::env::var("SUBSCRIPTION_TABLE_NAME").expect("SUBSCRIPTION_TABLE_NAME must be set");
    let subscription = client
        .get_item()
        .table_name(subscription_table)
        .key("pk", AttributeValue::S(request.user_id.to_string()))
        .projection_expression("streams")
        .send()
        .await?;
    Ok(subscription)
}

pub async fn add_stream(
    client: &aws_sdk_dynamodb::Client,
    request: &StreamRequest,
) -> Result<(), ApplicationError> {
    let streaming_table =
        std::env::var("STREAMING_TABLE_NAME").expect("STREAMING_TABLE_NAME must be set");
    client
        .put_item()
        .table_name(streaming_table)
        .item("pk", AttributeValue::S(request.user_id.to_string()))
        .item("sk", AttributeValue::S(request.video_id.to_string()))
        .item(
            "ttlExpireAt",
            AttributeValue::N(format!(
                "{:}",
                (chrono::Utc::now() + chrono::Duration::seconds(60)).timestamp()
            )),
        )
        .send()
        .await?;

    Ok(())
}

async fn update_ttl(
    client: &aws_sdk_dynamodb::Client,
    request: &StreamRequest,
) -> Result<(), ApplicationError> {
    let streaming_table =
        std::env::var("STREAMING_TABLE_NAME").expect("STREAMING_TABLE_NAME must be set");
    client
        .update_item()
        .table_name(streaming_table)
        .key("pk", AttributeValue::S(request.user_id.to_string()))
        .key("sk", AttributeValue::S(request.video_id.to_string()))
        .update_expression("set ttlExpireAt = :ttlExpireAt")
        .expression_attribute_values(
            ":ttlExpireAt",
            AttributeValue::N(format!(
                "{:}",
                (chrono::Utc::now() + chrono::Duration::seconds(60)).timestamp()
            )),
        )
        .condition_expression("attribute_exists(pk)")
        .return_values(ReturnValue::UpdatedNew)
        .send()
        .await?;
    Ok(())
}

fn response(status_code: StatusCode, body: String) -> Response<String> {
    Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(body)
        .unwrap()
}
