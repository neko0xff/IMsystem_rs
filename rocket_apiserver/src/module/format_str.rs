use chrono::{DateTime, Local};

/*時問格式化輸出*/
pub fn console_time() -> String {
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();

    custom_format
}