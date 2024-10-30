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
    service::*,
    todo::*,
    websocket::*
};
use module::{
    database::AppState,
    format_str::console_time
};

/*主程式 */
#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let app_state = AppState::new().await;

    println!("[{}] API Server is starting.....",console_time());
    /*路由導向*/
    rocket::build()
        .manage(app_state)
        .mount("/", routes![index])
        .mount("/service",routes![
            healthcheck,
            test_db,
            webhook_check
        ])
        .mount("/api",routes![
            get_message,
            create_message,
            message_urlencoded,
            add_todo,
            remove_todo,
            list_todos
        ])
        .mount("/auth",routes![
            auth_login,
            auth_signup,
            auth_mailcheck,
            auth_forget,
            auth_updateusermeta
        ])
        .mount("/echo_ws", routes![
            echo_channel,
            echo_stream,
            echo_compose
        ])
}
