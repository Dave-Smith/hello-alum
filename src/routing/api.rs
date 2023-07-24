use axum::{Router, routing::{get, post}, Json, extract::State};
use crate::routing::state::{Todo, AppState};

use super::state::CreateTodo;

async fn list_todos(State(app_state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = app_state.todos.lock().unwrap();
    Json(todos.clone())
}

async fn create_todo(State(app_state): State<AppState>, Json(create_todo): Json<CreateTodo>) -> Json<Todo> {
    let mut todos = app_state.todos.lock().unwrap();
    let new_todo = Todo {
        title: create_todo.title,
        completed: false
    };
    todos.push(new_todo.clone());
    Json(new_todo)
}

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_todos))
        .route("/", post(create_todo))

}