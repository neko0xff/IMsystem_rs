use crate::{
    ds::{ds_auth::*,ds_user::*},
    module::database::AppState
};
use chrono::{DateTime, Local};
use rocket::{
    State,
    http::Status,
    response::status::Custom,
    serde::json::Json
};
use sqlx::query_as;
use argon2::{
    self,
    Argon2,
    PasswordVerifier,
    PasswordHash,
    password_hash::{
        SaltString,
        PasswordHasher
    }
};
use rand_core::OsRng;

#[post("/login", format = "json", data = "<meta>")]
// body: raw (Content-Type: application/json)
pub async fn auth_login(
    meta: Json<InputmetaLogin>,
    state: &State<AppState>
) -> Result<Json<Outputmeta>, Custom<String>> {
    let metadata = meta.into_inner();
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
    let db = &state.db;
    let user = query_as!(
        User,
        "SELECT id, username, password, mail,loginname FROM users WHERE username = ?",
        metadata.username
    )
        .fetch_one(db)
        .await
        .map_err(|_| Custom(Status::Unauthorized, "Invalid username or password".to_string()))?;
    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|_| Custom(Status::Unauthorized, "Invalid username or password".to_string()))?;
    let argon2 = Argon2::default();
    let is_valid = argon2
        .verify_password(metadata.password.as_bytes(), &parsed_hash)
        .is_ok();
    
    println!("[{}] POST /auth/login", custom_format);

    if is_valid {
        let output =  Outputmeta {
            status : "1".to_string(),
            message : "Is Finnish Login!".to_string(),
            username :  user.username.clone(),
            loginname: user.loginname.clone(),
            mail: user.mail.clone()
        };
        Ok(Json(output))
    } else {
        Err(Custom(Status::Unauthorized, "Invalid username or password".to_string()))
    }
}

#[post("/signup", format = "json", data = "<meta>")]
// body: raw (Content-Type: application/json)
pub async fn auth_signup(
    meta: Json<InputmetaSingup>,
    state: &State<AppState>
) -> Result<Json<Outputmeta>, Custom<String>> {
    let metadata = meta.into_inner();
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
    let db = &state.db;
    let existing_user = query_as!(
            User,
            "SELECT id, username, password, mail,loginname FROM users WHERE username = ?",
            metadata.username
        )
        .fetch_optional(db)
        .await
        .map_err(|e| Custom(Status::InternalServerError, format!("Database error: {}", e)))?;
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2.hash_password(
            metadata.password.as_bytes(),
            &salt,
         )
        .map_err(|e| Custom(Status::InternalServerError, format!("Password hashing error: {}", e)))?
        .to_string();
    let loginname_set = metadata.username.clone(); // default: use "username"
    let user = User {
        id: 0,
        username: metadata.username,
        loginname: loginname_set,
        password: password_hash,
        mail: metadata.mail
    };
    let output =  Outputmeta {
        status : "1".to_string(),
        message : "Is Finnish Add!".to_string(),
        username : user.username.clone(),
        loginname: user.loginname.clone(),
        mail: user.mail.clone()
    };
    
    println!("[{}] POST /auth/signup", custom_format);
    if let Some(_) = existing_user {
        return Err(Custom(Status::Conflict, "Username already exists".to_string()));
    }

    sqlx::query("INSERT INTO users (username, password, mail,loginname) VALUES (?, ?, ?,?)")
        .bind(&user.username)
        .bind(&user.password)
        .bind(&user.mail)
        .bind(&user.username)
        .execute(db)
        .await
        .map_err(|e| Custom(Status::InternalServerError, format!("Database error: {}", e)))?;

    Ok(Json(output))
}

#[post("/mailcheck", format = "json", data = "<meta>")]
// body: raw (Content-Type: application/json)
pub async fn auth_mailcheck(
    meta: Json<InputmetaMailcheck>,
    state: &State<AppState>
) -> Result<Json<Outputmeta>, Custom<String>> {
    let metadata = meta.into_inner();
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
    let db = &state.db;
    let user = query_as!(
        UserSearch,
        "SELECT id, username,mail,loginname FROM users WHERE mail = ?",
        metadata.mail
    )
        .fetch_one(db)
        .await
        .map_err(|_| Custom(Status::Unauthorized, "Invalid username or mail".to_string()))?;
    
    println!("[{}] POST /auth/mailcheck", custom_format);

    let output =  Outputmeta {
            status : "1".to_string(),
            message : "Is Finnish check!".to_string(),
            username :  user.username.clone(),
            loginname: user.loginname.clone(),
            mail: user.mail.clone()
    };
     
    Ok(Json(output))
  
}

#[post("/forget", format = "json", data = "<meta>")]
// body: raw (Content-Type: application/json)
pub async fn auth_forget(
    meta: Json<InputmetaForget>,
    state: &State<AppState>
) -> Result<Json<Outputmeta>, Custom<String>> {
    let metadata = meta.into_inner();
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
    let db = &state.db;
    let existing_user = query_as!(
        User,
        "SELECT id, username, password, mail, loginname FROM users WHERE username = ?",
        metadata.username
    )
        .fetch_optional(db)
        .await
        .map_err(|e| Custom(Status::InternalServerError, format!("Database error: {}", e)))?;

    let existing_user = existing_user.unwrap();
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(metadata.password.as_bytes(), &salt)
        .map_err(|e| Custom(Status::InternalServerError, format!("Password hashing error: {}", e)))?
        .to_string();

    let user = User {
        id: existing_user.id,
        username: existing_user.username.clone(),
        loginname: existing_user.username.clone(),  // default: use "username"
        password: password_hash,
        mail: existing_user.mail.clone(),
    };

    println!("[{}] POST /auth/forget", custom_format);
    sqlx::query("UPDATE users SET password = ? WHERE username = ?")
        .bind(&user.password)
        .bind(&user.username)
        .execute(db)
        .await
        .map_err(|e| Custom(Status::InternalServerError, format!("Database error: {}", e)))?;

    let output = Outputmeta {
        status: "1".to_string(),
        message: "Is Finnish Update data!".to_string(),
        username: user.username,
        loginname: user.loginname,
        mail: user.mail,
    };

    Ok(Json(output))
}

#[post("/updateUserMeta", format = "json", data = "<meta>")]
// body: raw (Content-Type: application/json)
pub async fn auth_updateusermeta(
    meta: Json<InputmetaUpdate>,
    state: &State<AppState>
) -> Result<Json<Outputmeta>, Custom<String>> {
    let metadata = meta.into_inner();
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();
    let db = &state.db;
    let existing_user = query_as!(
        User,
        "SELECT id, username, password, mail, loginname FROM users WHERE username = ?",
        metadata.username
    )
        .fetch_optional(db)
        .await
        .map_err(|e| Custom(Status::InternalServerError, format!("Database error: {}", e)))?;
    let existing_user = existing_user.unwrap();
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = argon2
        .hash_password(metadata.password.as_bytes(), &salt)
        .map_err(|e| Custom(Status::InternalServerError, format!("Password hashing error: {}", e)))?
        .to_string();
    let user = User {
        id: existing_user.id,
        username: existing_user.username.clone(),
        loginname: metadata.loginname,  
        password: password_hash,
        mail: metadata.mail,
    };

    println!("[{}] POST /auth/updateUserMeta", custom_format);
    sqlx::query("UPDATE users SET password = ?,loginname = ?, mail = ? WHERE username = ?")
        .bind(&user.password)
        .bind(&user.loginname)
        .bind(&user.mail)
        .bind(&user.username)
        .execute(db)
        .await
        .map_err(|e| Custom(Status::InternalServerError, format!("Database error: {}", e)))?;

    let output = Outputmeta {
        status: "1".to_string(),
        message: "Is Finnish Update data!".to_string(),
        username: user.username,
        loginname: user.loginname,
        mail: user.mail,
    };

    Ok(Json(output))
}

