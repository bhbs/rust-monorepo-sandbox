use serde::{Deserialize, Serialize};

pub type Message = String;

#[derive(Debug, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MessageBody {
    pub Type: String,
    pub MessageId: String,
    pub TopicArn: String,
    pub Message: Message,
    pub Timestamp: String,
    pub SignatureVersion: String,
    pub Signature: String,
    pub SigningCertURL: String,
}
