use serde::{Serialize, Deserialize};
use sqlx::FromRow;

/*使用者*/
#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub loginname: String,
    pub mail: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSearch {
    pub id: i32,
    pub username: String,
    pub mail: String,
    pub loginname: String,
}


