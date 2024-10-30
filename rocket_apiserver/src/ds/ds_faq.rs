use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

/* 留言資料表 */
#[derive(Serialize, Deserialize, FromRow)]
pub struct GuestBook {
    pub id: u32, 
    pub title: String, 
    pub user_id: String, 
    pub date_time: NaiveDateTime, 
    pub visit: u32, 
    pub report: u32, 
    pub display: bool, 
    pub content: String, 
    pub description: String
}

/* 回答資料表 */
#[derive(Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: u32, 
    pub qid: u32, 
    pub manager: String, 
    pub date_time: NaiveDateTime, 
    pub content: String
}

/* 功能資料表 */
#[derive(Serialize, Deserialize, FromRow)]
pub struct Configure {
    pub id: u32, 
    pub sort: u32, 
    pub category: String, 
    pub title: String, 
    pub name: String, 
    pub value: String, 
    pub encode_name: String, 
    pub date_time: NaiveDateTime, 
    pub conf_type: String, 
    pub content: String, 
    pub description: String
}

/* 使用者記錄 */
#[derive(Serialize, Deserialize, FromRow)]
pub struct UserLog {
    pub id: u32, 
    pub user_level: String, 
    pub last_quest_id: u32, 
    pub ask_count: u32, 
    pub answer_count: u32, 
    pub last_login_time: NaiveDateTime, 
    pub description: String
}

/* 歷史(log)資料表 */
#[derive(Serialize, Deserialize, FromRow)]
pub struct WebLog {
    pub id: u32, 
    pub date_time: NaiveDateTime, 
    pub user_id: String, 
    pub action: String
}
