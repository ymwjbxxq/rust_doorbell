use aws_sdk_eventbridge::model::PutEventsRequestEntry;
use chrono::Duration;
use chrono::Utc;
use lambda_runtime::{handler_fn, Context, Error};
use rust_doorbell::aws::client::AWSClient;
use rust_doorbell::aws::client::AWSConfig;
use rust_doorbell::dtos::connected_event::ConnectedEvent;
use rust_doorbell::dtos::websocket_request::WebSocketRequest;
use rust_doorbell::error::ApplicationError;
use rust_doorbell::models::connection::Connection;
use rust_doorbell::utils::*;
use serde_json::{json, Value};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();
    // Initialize AWS client
    let config = aws_config::load_from_env().await;
    let config = AWSConfig::set_config(config);
    let aws_client = config.on_connect();

    lambda_runtime::run(handler_fn(|event: WebSocketRequest, ctx: Context| {
        execute(&aws_client, event, ctx)
    }))
    .await?;
    Ok(())
}
ge
pub async fn execute(aws_client: &AWSClient, event: WebSocketRequest, _ctx: Context) -> Result<Value, ApplicationError> {
    info!("EVENT {:?}", event);

    let connection = Connection {
        connection_id: event.request_context.connection_id.clone(),
        ttl_expire_at: (Utc::now() + Duration::seconds(120)).timestamp(),
    };

    add_connection(&aws_client, connection).await?;
    send_event(&aws_client, &event).await?;

    Ok(json!({
        "statusCode": 200
    }))
}

async fn add_connection(aws_client: &AWSClient, connection: Connection) -> Result<(), ApplicationError> {
    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    let _res = aws_client.dynamo_db_client.as_ref().unwrap()
        .put_item()
        .table_name(&table_name)
        .set_item(Some(connection.to_dynamodb()))
        .send()
        .await?;

    Ok(())
}

async fn send_event(aws_client: &AWSClient, event: &WebSocketRequest) -> Result<(), ApplicationError> {
    let bus_name = std::env::var("EVENT_BUS_NAME").expect("EVENT_BUS_NAME must be set");

    let message = ConnectedEvent {
        connection_id: event.request_context.connection_id.clone(),
    };
    let put_events_request_entry = PutEventsRequestEntry::builder()
        .event_bus_name(bus_name)
        .source("doorbell.onconnect")
        .detail_type("connected")
        .detail(serde_json::to_string(&message)?)
        .build();

    let result = aws_client.event_bridge.as_ref().unwrap()
        .put_events()
        .entries(put_events_request_entry)
        .send()
        .await?;

    info!("EventBridge {:?}", result);
    Ok(())
}
