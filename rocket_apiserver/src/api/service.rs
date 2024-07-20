use crate::{
    ds::ds_ex::*,
    module::{
        database::AppState,
        format_str::console_time
    }
};
use rocket::{
    http::Status,
    serde::json::Json, 
    State
};
use sqlx::query_as;

#[get("/testDB")]
pub async fn test_db(state: &State<AppState>) -> Json<CustomResponse> {
    let result = query_as!(Message, "SELECT 1 AS id, 'Hello, world!' AS content")
        .fetch_one(&state.db)
        .await;

    println!("[{}] GET /service/testDB", console_time());
    /*SQL查詢*/
    match result {
        Ok(message) => Json(CustomResponse {
            status: "success".to_string(),
            message: "Query executed successfully".to_string(),
            data: Some(message),
            time: console_time()
        }),
        Err(e) => Json(CustomResponse {
            status: "error".to_string(),
            message: format!("Query failed: {}", e),
            data: None,
            time: console_time()
        }),
    }
}

#[get("/healthcheck")]
pub fn healthcheck() -> Json<serde_json::Value> {
    let json_response = serde_json::json!({
        "status": "1",
        "message": "API Services is Running",
        "time": console_time()
    });
    
    /*回傳*/
    println!("[{}] GET /service/healthcheck", console_time());
    Json(json_response)
}

#[post("/webhook", format = "application/json", data = "<payload>")]
pub async fn webhook_check(payload: Json<WebhookPayload>) -> (Status, Json<serde_json::Value>) {
    let  json_response = serde_json::json!({
        "status": "1",
        "message": "webhook is running",
        "time": console_time()
    });
 
    /*回傳部分*/
    println!("[{}] POST /service/webhook", console_time());
    println!("Webhook= {:?}", payload);

    (Status::Ok,Json(json_response))
}

