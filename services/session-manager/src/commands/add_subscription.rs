use crate::{error::ApplicationError, models::subscription_request::SubscriptionRequest};
use async_trait::async_trait;
use aws_sdk_dynamodb::{ model::AttributeValue, Client };

#[async_trait]
pub trait AddSubscriptionCommanad {
    async fn new() -> Self;
    async fn execute(&self, client: &Client, request: &SubscriptionRequest) -> Result<(), ApplicationError>;
}

#[derive(Debug)]
pub struct AddSubscription {
    table_name: String,
}

#[async_trait]
impl AddSubscriptionCommanad for AddSubscription {
    async fn new() -> Self {
        let table_name =
            std::env::var("SUBSCRIPTION_TABLE_NAME").expect("STREAMING_TABLE_NAME must be set");
        Self { table_name }
    }

    async fn execute( &self, client: &Client, request: &SubscriptionRequest) -> Result<(), ApplicationError> {
        client
            .put_item()
            .table_name(&self.table_name)
            .item("pk", AttributeValue::S(request.user_id.to_string()))
            .item("plan_id", AttributeValue::S(request.plan_id.to_string()))
            .item(
                "devices",
                AttributeValue::N(format!("{:}", request.devices)),
            )
            .item(
                "streams",
                AttributeValue::N(format!("{:}", request.streams)),
            )
            .condition_expression("attribute_not_exists(pk)")
            .send()
            .await?;
        Ok(())
    }
}
