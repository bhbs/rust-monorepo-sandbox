use anyhow::Result;
use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use common::Message;
use serde::Deserialize;
use tokio::signal::unix::{signal, SignalKind};

#[derive(Deserialize)]
struct PublishRequest {
    message: String,
}

async fn index() -> &'static str {
    "Service A is running"
}

async fn publish_message(
    Extension(sns_client): Extension<aws_sdk_sns::Client>,
    Json(payload): Json<PublishRequest>,
) -> impl IntoResponse {
    let msg: Message = payload.message;

    match sns_client
        .publish()
        .topic_arn("arn:aws:sns:us-east-1:000000000000:service_a_topic")
        .message(msg)
        .send()
        .await
    {
        Ok(_) => "Message published",
        Err(_) => "Failed to publish message",
    }
}
#[tokio::main]
async fn main() -> Result<()> {
    let config = aws_config::from_env()
        .endpoint_url("http://localstack:4566")
        .load()
        .await;
    let sns_client = aws_sdk_sns::Client::new(&config);

    let app = Router::new()
        .route("/", get(index))
        .route("/publish", post(publish_message))
        .layer(Extension(sns_client));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    signal(SignalKind::terminate())
        .expect("Failed to register signal handler")
        .recv()
        .await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_service_a() {
        assert_eq!(1 + 2, 3);
    }
}
