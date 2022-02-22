use crate::{error::ApplicationError, models::device_request::DeviceRequest,};
use async_trait::async_trait;
use aws_sdk_dynamodb::{
    model::{AttributeValue, ReturnValue},
    Client, output::UpdateItemOutput,
};

#[async_trait]
pub trait UpdateSubscriptionDeviceCommanad {
    async fn new() -> Self;
    async fn execute(&self, client: &Client, request: &DeviceRequest) -> Result<UpdateItemOutput, ApplicationError>;
}

#[derive(Debug)]
pub struct UpdateSubscriptionDevice {
    table_name: String,
}

#[async_trait]
impl UpdateSubscriptionDeviceCommanad for UpdateSubscriptionDevice {
    async fn new() -> Self {
        let table_name =
            std::env::var("SUBSCRIPTION_TABLE_NAME").expect("STREAMING_TABLE_NAME must be set");
        Self { table_name }
    }

    async fn execute( &self, client: &Client, request: &DeviceRequest) -> Result<UpdateItemOutput, ApplicationError> {
        let result = client
            .update_item()
            .table_name(&self.table_name)
            .key("pk", AttributeValue::S(request.user_id.to_string()))
            .update_expression("set devices = devices - :incr")
            .expression_attribute_values(
                ":incr",
                AttributeValue::N(format!("{:}", request.device_count)),
            )
            .expression_attribute_values(":limit", AttributeValue::N(format!("{:}", 0)))
            .condition_expression("attribute_exists(pk) AND devices > :limit")
            .return_values(ReturnValue::UpdatedNew)
            .send()
            .await?;
        Ok(result)
    }
}
