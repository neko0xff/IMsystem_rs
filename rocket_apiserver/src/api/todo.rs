use crate::{
    ds::ds_todo::*,
    module::database::AppState,
    module::format_str::console_time
};
use rocket::{
    serde::json::Json, 
    State,
    http::Status
};

#[post("/todo/add", format = "json", data = "<new_todo>")]
pub async fn add_todo(
    new_todo: Json<InputmetaTodo>, 
    state: &State<AppState>
) -> Result<Json<Todo>, Status> {
    let new_todo = new_todo.into_inner();
    let todo = Todo::create(new_todo.username, new_todo.description, state)
        .await
        .map_err(|_| Status::InternalServerError)?;
    println!("[{}] POST /api/todo/add", console_time());
    Ok(Json(todo))
}

#[delete("/todo/remove/<id>")]
pub async fn remove_todo(
    id: i32, 
    state: &State<AppState>
) -> Result<Json<bool>, Status> {
    let removed = Todo::delete(id, state).await
        .map_err(|_| Status::InternalServerError)?;
    println!("[{}] DELETE /api/todo/remove/{}", console_time(),id);
    Ok(Json(removed))
}

#[get("/todo/list")]
pub async fn list_todos(
    state: &State<AppState>
) -> Result<Json<Vec<Todo>>, Status> {
    let todos = Todo::list(state).await
        .map_err(|_| Status::InternalServerError)?;
    println!("[{}] GET /api/todo/list", console_time());
    Ok(Json(todos))
}