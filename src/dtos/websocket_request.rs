use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WebSocketRequest {
    pub request_context: RequestContext,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestContext {
    pub event_type: EventType,
    pub connection_id: String,
    pub domain_name: String,
    pub stage: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventType {
    Connect,
    Disconnect,
}