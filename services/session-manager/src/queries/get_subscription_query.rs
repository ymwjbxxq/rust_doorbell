use async_trait::async_trait;
use aws_sdk_dynamodb::{model::AttributeValue, Client, output::GetItemOutput};
use crate::{models::stream_request::StreamRequest, error::ApplicationError};

#[async_trait]
pub trait GetSubscriptionQuery {
    async fn new() -> Self;
    async fn execute(&self, client: &Client, request: &StreamRequest) -> Result<GetItemOutput, ApplicationError>;
}

#[derive(Debug)]
pub struct GetSubscription {
  table_name: String,
}

#[async_trait]
impl GetSubscriptionQuery for GetSubscription {
    async fn new() -> Self {
        let table_name = std::env::var("SUBSCRIPTION_TABLE_NAME").expect("SUBSCRIPTION_TABLE_NAME must be set");
        Self { table_name }
    }

    async fn execute(&self, client: &Client, request: &StreamRequest) -> Result<GetItemOutput, ApplicationError> {
        let subscription = client
            .get_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(request.user_id.to_string()))
            .projection_expression("streams")
            .send()
            .await?;
        Ok(subscription)
    }
}
