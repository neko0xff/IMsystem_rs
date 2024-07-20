use crate::module::format_str::console_time;

#[get("/")]
pub fn index() -> &'static str {
    println!("[{}] GET /", console_time());
    "API Server is Running!" // 在網頁上顯示
}
