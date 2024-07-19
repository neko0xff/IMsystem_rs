use serde::{Serialize,Deserialize};

/*查詢欄位*/
#[derive(FromForm, Serialize, Deserialize)]
pub struct InputmetaLogin {
    pub username: String,
    pub password: String,
}

#[derive(FromForm, Serialize, Deserialize)]
pub struct InputmetaSingup {
    pub username: String,
    pub password: String,
    pub mail: String
}

#[derive(FromForm, Serialize, Deserialize)]
pub struct InputmetaForget {
    pub username: String,
    pub password: String,
}

#[derive(FromForm, Serialize, Deserialize)]
pub struct InputmetaUpdate {
    pub username: String,
    pub password: String,
    pub loginname: String,
    pub mail: String
}

#[derive(FromForm, Serialize, Deserialize)]
pub struct InputmetaMailcheck {
    pub mail: String
}

/*輸出資訊*/
#[warn(non_camel_case_types)]
#[derive(Serialize, Deserialize)]
pub struct  Outputmeta {
    pub status: String,
    pub message: String,
    pub username: String,
    pub loginname: String,
    pub mail: String
}