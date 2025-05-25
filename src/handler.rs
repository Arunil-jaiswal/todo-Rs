use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::ToDo,
    schema::{CreatetodoSchema, FilterOptions, UpdatetodoSchema},
    AppState,
};


pub async fn todo_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();


    let query_result = sqlx::query_as!(
        ToDo,
        "SELECT * FROM todos ORDER by id",
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all todo items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let todos = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": todos.len(),
        "todos": todos
    });
    Ok(Json(json_response))
}

pub async fn create_todo_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreatetodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        ToDo,
        "INSERT INTO todos (title,content) VALUES ($1, $2) RETURNING *",
        body.title.to_string(),
        body.content.to_string()
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(todo) => {
            let todo_response = json!({"status": "success","data": json!({
                "todo": todo
            })});

            return Ok((StatusCode::CREATED, Json(todo_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "todo with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

pub async fn get_todo_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ToDo, "SELECT * FROM todos WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(todo) => {
            let todo_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "todo": todo
            })});

            return Ok(Json(todo_response));
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "fail",
                "message": format!("todo with ID: {} not found", id)
            });
            return Err((StatusCode::NOT_FOUND, Json(error_response)));
        }
    }
}

pub async fn edit_todo_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdatetodoSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ToDo, "SELECT * FROM todos WHERE id = $1", id)
        .fetch_one(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("todo with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }


    let todo = query_result.unwrap();

    let query_result = sqlx::query_as!(
        ToDo,
        "UPDATE todos SET title = $1, content = $2 WHERE id = $3 RETURNING *",
        body.title.to_owned().unwrap_or(todo.title),
        body.content.to_owned().unwrap_or(todo.content),
        id
    )
    .fetch_one(&data.db)
    .await
    ;

    match query_result {
        Ok(todo) => {
            let todo_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "todo": todo
            })});

            return Ok(Json(todo_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

pub async fn delete_todo_handler(
    Path(id): Path<uuid::Uuid>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM todos  WHERE id = $1", id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("todo with ID: {} not found", id)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    Ok(StatusCode::NO_CONTENT)
}