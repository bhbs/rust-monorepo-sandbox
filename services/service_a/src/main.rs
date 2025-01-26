use actix_web::{web, App, HttpServer, Responder};
use common::Message;
use serde::Deserialize;

#[derive(Deserialize)]
struct PublishRequest {
    message: String,
}

async fn index() -> impl Responder {
    "Service A is running"
}

async fn publish_message(
    sns_client: web::Data<aws_sdk_sns::Client>,
    req: web::Json<PublishRequest>,
) -> impl Responder {
    let msg: Message = req.message.clone();

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
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = aws_config::from_env()
        .endpoint_url("http://localstack:4566")
        .load()
        .await;
    let sns_client = aws_sdk_sns::Client::new(&config);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(sns_client.clone()))
            .route("/", web::get().to(index))
            .route("/publish", web::post().to(publish_message))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
