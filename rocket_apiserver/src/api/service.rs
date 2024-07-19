use crate::{
    ds::ds_ex::*,
    module::database::AppState
};
use rocket::{
    http::Status,
    serde::json::Json, 
    State
};
use chrono::{DateTime, Local};
use sqlx::query_as;

#[get("/testDB")]
pub async fn test_db(state: &State<AppState>) -> Json<CustomResponse> {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
    let result = query_as!(Message, "SELECT 1 AS id, 'Hello, world!' AS content")
        .fetch_one(&state.db)
        .await;

    println!("[{}] GET /service/testDB", custom_format);
    /*SQL查詢*/
    match result {
        Ok(message) => Json(CustomResponse {
            status: "success".to_string(),
            message: "Query executed successfully".to_string(),
            data: Some(message),
            time: custom_format
        }),
        Err(e) => Json(CustomResponse {
            status: "error".to_string(),
            message: format!("Query failed: {}", e),
            data: None,
            time:custom_format
        }),
    }
}

#[get("/healthcheck")]
pub fn healthcheck() -> Json<serde_json::Value> {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
    let json_response = serde_json::json!({
        "status": "1",
        "message": "API Services is Running",
        "time": custom_format
    });
    
    /*回傳*/
    println!("[{}] GET /service/healthcheck", custom_format);
    Json(json_response)
}

#[post("/webhook", format = "application/json", data = "<payload>")]
pub async fn webhook_check(payload: Json<WebhookPayload>) -> (Status, Json<serde_json::Value>) {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
    let  json_response = serde_json::json!({
        "status": "1",
        "message": "webhook is running",
        "time": custom_format
    });
 
    /*回傳部分*/
    println!("[{}] POST /service/webhook", custom_format);
    println!("Webhook= {:?}", payload);

    (Status::Ok,Json(json_response))
}

