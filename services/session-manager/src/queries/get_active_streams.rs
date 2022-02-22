use std::collections::HashMap;
use async_trait::async_trait;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use crate::{models::stream_request::StreamRequest, utils::dynamodb::AttributeValuesExt, error::ApplicationError};

#[async_trait]
pub trait GetActiveStreamsQuery {
    async fn new() -> Self;
    async fn execute(&self, client: &Client, request: &StreamRequest) -> Result<Vec<HashMap<String, AttributeValue>>, ApplicationError>;
}

#[derive(Debug)]
pub struct GetActiveStreams {
  table_name: String,
}

#[async_trait]
impl GetActiveStreamsQuery for GetActiveStreams {
  async fn new() -> Self {
    let table_name = std::env::var("STREAMING_TABLE_NAME").expect("STREAMING_TABLE_NAME must be set");
    Self { table_name }
  }

  async fn execute(&self, client: &Client, request: &StreamRequest) -> Result<Vec<HashMap<String, AttributeValue>>, ApplicationError> {
    let streams = client
        .query()
        .table_name(&self.table_name)
        .key_condition_expression("pk = :pk ")
        .expression_attribute_values(":pk", AttributeValue::S(request.user_id.to_string()))
        .send()
        .await?;
    
    let now = chrono::Utc::now().timestamp();
    let streams =  streams.items.unwrap()
        .into_iter()
        .filter(|row| row.get_number("ttlExpireAt").unwrap() > now)
        .collect::<Vec<_>>();
    Ok(streams)
  }
}
