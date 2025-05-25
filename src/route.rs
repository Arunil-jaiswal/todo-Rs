use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::{
        create_todo_handler, delete_todo_handler, edit_todo_handler, get_todo_handler,
        todo_list_handler,
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/todos/", post(create_todo_handler))
        .route("/api/todos", get(todo_list_handler))
        .route(
            "/api/todos/:id",
            get(get_todo_handler)
                .patch(edit_todo_handler)
                .delete(delete_todo_handler),
        )
        .with_state(app_state)
}