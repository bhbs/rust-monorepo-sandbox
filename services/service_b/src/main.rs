use std::{thread::sleep, time::Duration};

use anyhow::Result;
use common::{Message, MessageBody};
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() -> Result<()> {
    let config = aws_config::from_env()
        .endpoint_url("http://localstack:4566")
        .load()
        .await;
    let sqs_client = aws_sdk_sqs::Client::new(&config);

    let queue_url = "http://localstack:4566/000000000000/service_b_queue";

    let mut sigterm = signal(SignalKind::terminate())?;

    loop {
        tokio::select! {
            _ = sigterm.recv() => {
                break;
            },
            result = handle_sqs_message(&sqs_client, queue_url) => {
                if let Err(e) = result {
                    eprintln!("Error handling SQS message: {:?}", e);
                    sleep(Duration::from_secs(5))
                }
            }
        }
    }

    Ok(())
}

async fn handle_sqs_message(sqs_client: &aws_sdk_sqs::Client, queue_url: &str) -> Result<()> {
    match receive_message(sqs_client, queue_url).await {
        Ok(message) => {
            if let Some(message) = message {
                println!("Received message: {:?}", message);
            } else {
                println!("Polling SQS queue...");
            };
        }
        Err(e) => {
            eprintln!("Error receiving message: {:?}", e);
            return Err(e);
        }
    }

    Ok(())
}

async fn receive_message(
    sqs_client: &aws_sdk_sqs::Client,
    queue_url: &str,
) -> Result<Option<Message>> {
    let response = sqs_client
        .receive_message()
        .queue_url(queue_url)
        .max_number_of_messages(1)
        .wait_time_seconds(20)
        .send()
        .await?;

    if let Some(messages) = response.messages {
        if let Some(message) = messages.first() {
            delete_message(sqs_client, queue_url, message).await?;

            if let Some(body) = &message.body {
                let message_body: MessageBody = serde_json::from_str(body)?;
                return Ok(Some(message_body.Message.clone()));
            }
        }
    }

    Ok(None)
}

async fn delete_message(
    client: &aws_sdk_sqs::Client,
    queue_url: &str,
    message: &aws_sdk_sqs::types::Message,
) -> Result<()> {
    if let Some(receipt_handle) = message.receipt_handle() {
        client
            .delete_message()
            .queue_url(queue_url)
            .receipt_handle(receipt_handle)
            .send()
            .await?;
    } else {
        println!("No receipt handle found for the message.");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_service_b() {
        assert_eq!(1 + 2, 3);
    }
}
