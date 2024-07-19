use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    event: String,
    data: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub content: String,
}

#[derive(Serialize)]
pub struct CustomResponse {
    pub status: String,
    pub message: String,
    pub data: Option<Message>,
    pub time: String
}

#[derive(FromForm)]
pub struct MessageForm {
    pub id: u32,
    pub content: Option<String>,
}
