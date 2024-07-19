#[macro_use]
/*導入自定義函式庫*/
extern crate rocket;
mod api;
mod module;
mod ds;

/*引用函式庫 */
use api::{
    basic::*,
    message::*,
    auth::*,
    service::*
};
use module::database::AppState;
use chrono::{DateTime, Local};

/*主程式 */
#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let app_state = AppState::new().await;
    let current_local: DateTime<Local> = Local::now();
    let custom_format = current_local.format("%Y/%m/%d %H:%M:%S").to_string();

    println!("[{}] API Server is starting.....", custom_format);
    /*路由導向*/
    rocket::build()
        .manage(app_state)
        .mount("/", routes![index])
        .mount("/service",routes![healthcheck, test_db,webhook_check])
        .mount("/api",routes![get_message, create_message, message_urlencoded])
        .mount("/auth",routes![auth_login,auth_signup,auth_mailcheck,auth_forget,auth_updateusermeta])
}
