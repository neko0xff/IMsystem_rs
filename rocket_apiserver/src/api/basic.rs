use chrono::{DateTime, Local};

#[get("/")]
pub fn index() -> &'static str {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();

    println!("[{}] GET /", custom_format);
    "API Server is Running!" // 在網頁上顯示
}
