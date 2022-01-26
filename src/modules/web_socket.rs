use async_trait::async_trait;
use aws_sdk_apigatewaymanagement::{config, Blob, Client, Endpoint};
use serde::{Deserialize, Serialize};
use crate::{dtos::compare_face_dto::ResponseType, error::ApplicationError};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PostWebSocketRequest {
    pub connection_id: String,
    pub blob: ResponseType,
}

#[async_trait]
pub trait Socket {
    async fn new(config: &aws_types::config::Config) -> Self;
    async fn post(self, request: &PostWebSocketRequest) -> Result<(), ApplicationError>;
}

pub struct WebSocket {
    api_management_config: aws_sdk_apigatewaymanagement::Config,
}

#[async_trait]
impl Socket for WebSocket {
    async fn new(config: &aws_types::config::Config) -> Self {
        let domain = std::env::var("WSS_DOMAIN").expect("WSS_DOMAIN must be set");
        let endpoint = Endpoint::immutable(domain.parse().unwrap());
        let api_management_config = config::Builder::from(config)
            .endpoint_resolver(endpoint)
            .build();
        Self { 
          api_management_config
        }
    }

    async fn post(self, request: &PostWebSocketRequest) -> Result<(),ApplicationError> {
        Client::from_conf(self.api_management_config)
            .post_to_connection()
            .connection_id(&request.connection_id)
            .data(Blob::new(serde_json::to_string(&request.blob).unwrap().as_bytes()))
            .send()
            .await?;

        Ok(())
    }
}

