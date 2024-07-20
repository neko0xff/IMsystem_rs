use crate::module::database::AppState;
use serde::{Serialize, Deserialize};
use sqlx::{FromRow, query_as, query};
use rocket::{
    http::Status, 
    response::status::Custom, 
    State
};

/*To do list */
#[derive(Deserialize)]
pub struct InputmetaTodo {
    pub username: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct OutputmetaTodo {
    pub status: String,
    pub message: String,
    pub time: String
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub username: String,
    pub description: String,
}

impl Todo {
    pub async fn create(username: String, description: String, state: &State<AppState>
    ) -> Result<Self, Custom<String>> {
       
        let result = query!(
            "INSERT INTO todos(username, description) VALUES (?, ?)",
            username,description )
            .execute(&state.db)
            .await
            .map_err(|e| Custom(Status::InternalServerError, format!("Database insert error: {}", e)))?;
        let id = result.last_insert_id() as i32;
        let todo = query_as!(
            Todo,
            "SELECT id, username, description FROM todos WHERE id = ? ",
            id
        )
        .fetch_one(&state.db)
        .await
        .map_err(|e| Custom(Status::InternalServerError, format!("Database select error: {}", e)))?;
    
        Ok(todo)
    }

    pub async fn delete(id: i32, state: &State<AppState>
    ) -> Result<bool, sqlx::Error> {
        let result = query!(
            "DELETE FROM todos WHERE id = ?",
            id
        )
        .execute(&state.db)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn list(state: &State<AppState>
    ) -> Result<Vec<Self>, sqlx::Error> {
        query_as!(
            Todo,
            "SELECT id, username, description FROM todos"
        )
        .fetch_all(&state.db)
        .await
    }
}