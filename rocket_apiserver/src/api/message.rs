use crate::ds::ds_ex::*;
use chrono::{DateTime, Local};
use rocket::{form::Form,serde::json::Json};

#[get("/message/<id>")]
pub fn get_message(id: i32) -> Json<Message> {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
    let message = Message {
        id,
        content: format!("This is message number {}", id),
    };

    println!("[{}] GET /api/message/<id>", custom_format);
    Json(message)
}

#[post("/message", format = "json", data = "<message>")]
// body: raw (Content-Type: application/json)
pub fn create_message(message: Json<Message>) -> Json<Message> {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();

    println!("[{}] POST /api/message", custom_format);
    Json(Message {
        id: message.id,
        content: message.content.clone(),
    })
}

#[post("/message_urlencoded", data = "<message_form>")]
// body: x-www-form-urlencoded
pub fn message_urlencoded(message_form: Form<MessageForm>) -> Json<Message> {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
    let message = Message {
        id: message_form.id as i32,
        content: message_form
            .content
            .clone()
            .unwrap_or_else(|| "No content provided".to_string()),
    };

    println!("[{}] POST /message_urlencoded", custom_format);
    Json(message)
}
