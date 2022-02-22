use crate::{error::ApplicationError, models::stream_request::StreamRequest};
use async_trait::async_trait;
use aws_sdk_dynamodb::{
    model::{AttributeValue, ReturnValue},
    Client,
};

#[async_trait]
pub trait UpdateStreamsTTLCommanad {
    async fn new() -> Self;
    async fn execute(&self, client: &Client, request: &StreamRequest) -> Result<(), ApplicationError>;
}

#[derive(Debug)]
pub struct UpdateStreamsTTL {
    table_name: String,
}

#[async_trait]
impl UpdateStreamsTTLCommanad for UpdateStreamsTTL {
    async fn new() -> Self {
        let table_name =
            std::env::var("STREAMING_TABLE_NAME").expect("STREAMING_TABLE_NAME must be set");
        Self { table_name }
    }

    async fn execute(
        &self, client: &Client, request: &StreamRequest) -> Result<(), ApplicationError> {
        client
            .update_item()
            .table_name(&self.table_name)
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
}
