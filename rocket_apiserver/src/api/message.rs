use crate::{
    ds::ds_ex::*,
    module::format_str::console_time
};
use rocket::{form::Form,serde::json::Json};

#[get("/message/<id>")]
pub fn get_message(id: i32) -> Json<Message> {
    let message = Message {
        id,
        content: format!("This is message number {}", id),
    };

    println!("[{}] GET /api/message/<{}>", console_time(),id);
    Json(message)
}

#[post("/message", format = "json", data = "<message>")]
// body: raw (Content-Type: application/json)
// 傳送內容: json
pub fn create_message(message: Json<Message>) -> Json<Message> {
    println!("[{}] POST /api/message", console_time());
    Json(Message {
        id: message.id,
        content: message.content.clone(),
    })
}

#[post("/message_urlencoded", data = "<message_form>")]
// body: x-www-form-urlencoded
// 傳送內容: Key1=[data1]&Key2=[data2]
pub fn message_urlencoded(message_form: Form<MessageForm>) -> Json<Message> {
    let message = Message {
        id: message_form.id as i32,
        content: message_form
            .content
            .clone()
            .unwrap_or_else(|| "No content provided".to_string()),
    };

    println!("[{}] POST /message_urlencoded", console_time());
    Json(message)
}
