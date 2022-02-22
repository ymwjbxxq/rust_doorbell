use crate::{error::ApplicationError, models::stream_request::StreamRequest};
use async_trait::async_trait;
use aws_sdk_dynamodb::{model::AttributeValue, Client,};

#[async_trait]
pub trait AddStreamCommanad {
    async fn new() -> Self;
    async fn execute(&self, client: &Client, request: &StreamRequest) -> Result<(), ApplicationError>;
}

#[derive(Debug)]
pub struct AddStream {
    table_name: String,
}

#[async_trait]
impl AddStreamCommanad for AddStream {
    async fn new() -> Self {
        let table_name =
            std::env::var("STREAMING_TABLE_NAME").expect("STREAMING_TABLE_NAME must be set");
        Self { table_name }
    }

    async fn execute(
        &self, client: &Client, request: &StreamRequest) -> Result<(), ApplicationError> {
        client
            .put_item()
            .table_name(&self.table_name)
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
}
