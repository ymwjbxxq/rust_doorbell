use lambda_runtime::{handler_fn, Context, Error};
use rust_doorbell::modules::web_socket::{PostWebSocketRequest, Socket, WebSocket};
use rust_doorbell::utils::*;
use rust_doorbell::{
    dtos::compare_face_dto::{ResponseType, SendCodeRequest, SendCodeResponse},
    error::ApplicationError,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_tracing();
    // Initialize AWS client
    let config = aws_config::load_from_env().await;

    lambda_runtime::run(handler_fn(|event: SendCodeRequest, ctx: Context| {
        execute(&config, event, ctx)
    }))
    .await?;
    Ok(())
}

pub async fn execute(config: &aws_types::config::Config, event: SendCodeRequest, _ctx: Context) -> Result<String, ApplicationError> {
    info!("EVENT {:?}", event);
    let v: Vec<&str> = event.input.split('/').collect();
    post_to_web_socket(&v[1].to_string(), &event.code, config).await?;

    Ok(String::from("sent"))
}

async fn post_to_web_socket(connection_id: &String, code: &String, config: &aws_types::config::Config) -> Result<(), ApplicationError> {
    let response = ResponseType::Code(SendCodeResponse {
        code: code.to_string(),
    });

    WebSocket::new(config)
        .await
        .post(&PostWebSocketRequest {
            connection_id: connection_id.to_string(),
            blob: response,
        })
        .await?;

    Ok(())
}
