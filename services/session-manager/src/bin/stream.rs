use aws_config::{RetryConfig, TimeoutConfig};
use lambda_http::{http::StatusCode, service_fn, Error, IntoResponse, Request, RequestExt};
use serde_json::json;
use session_manager::{
    commands::{
        add_stream::{AddStream, AddStreamCommanad},
        update_streams_ttl::{UpdateStreamsTTL, UpdateStreamsTTLCommanad},
    },
    models::stream_request::StreamRequest,
    queries::{
        get_active_streams::{GetActiveStreams, GetActiveStreamsQuery},
        get_subscription_query::{GetSubscription, GetSubscriptionQuery},
    },
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
    let body = event.payload::<StreamRequest>()?;

    if let Some(request) = body {
        let subscription = GetSubscription::new()
            .await
            .execute(client, &request)
            .await?;
        if let Some(subscription) = subscription.item {
            let allowed_streams = subscription.get_number("streams").unwrap();
            let streams = GetActiveStreams::new()
                .await
                .execute(client, &request)
                .await?;
            let current_streams = streams.len();
            let found = streams.into_iter().any(|row| {
                row.get_string("pk").unwrap() == request.user_id
                    && row.get_string("sk").unwrap() == request.video_id
            });

            if found {
                UpdateStreamsTTL::new()
                    .await
                    .execute(client, &request)
                    .await?;
                return Ok(ApiHelper::response(
                    StatusCode::OK,
                    json!({ "message": "ttl updated" }).to_string(),
                ));
            }

            if usize::try_from(allowed_streams).ok().unwrap() == current_streams {
                return Ok(ApiHelper::response(
                    StatusCode::NOT_ACCEPTABLE,
                    json!({ "message": "Too many active streams" }).to_string(),
                ));
            }
            AddStream::new().await.execute(client, &request).await?;
            return Ok(ApiHelper::response(
                StatusCode::CREATED,
                json!({ "message": "Stream added" }).to_string(),
            ));
        }
        return Ok(ApiHelper::response(
            StatusCode::INTERNAL_SERVER_ERROR,
            json!({ "message": "No subscription found" }).to_string(),
        ));
    }

    return Ok(ApiHelper::response(
        StatusCode::BAD_REQUEST,
        json!({ "message": "Failed to parse streams from request body" }).to_string(),
    ));
}
