use axum::{
    extract::{State , Json , Path},
    routing::{get , post , delete}, 
    Router,
    http::StatusCode,
};
use serde::{Deserialize , Serialize};
use tokio::net::TcpListener;

use std::sync::{Arc , Mutex};
use uuid::Uuid;
use std::env;

#[derive(Clone , Debug , Serialize , Deserialize)]
struct Todo {
    id :Uuid ,
    title : String 
}

#[derive(Debug , Deserialize)]
struct CreateTodo {
    title : String ,
}

type SharedState = Arc<Mutex<Vec<Todo>>>;

async fn get_todos(State(state) : State<SharedState>) -> Json<Vec<Todo>>{
    let todos = state.lock().unwrap();
    Json(todos.clone())
}

async fn create_todo(State(state) : State<SharedState> , Json(payload) : Json<CreateTodo>) -> Json<Todo> {
    let mut todos = state.lock().unwrap();
    let todo = Todo {
        id : Uuid::new_v4() ,
        title : payload.title,
    };

    todos.push(todo.clone());
    Json(todo)
}

async fn delete_todo(State(state) : State<SharedState> , Path(todo_id) : Path<Uuid>) -> Result<Json<String>, StatusCode> {
    let mut todos = state.lock().unwrap();
    let initial_length = todos.len();

    todos.retain(|todo| todo.id != todo_id);

    if todos.len() < initial_length {
        Ok(Json("Todo deleted successfully".to_string()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[tokio::main]

async fn main (){
    let state = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new().route("/todos" , get(get_todos).post(create_todo)).route("/todos/{id}" , delete(delete_todo)).with_state(state);

    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    println!("Server on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener , app ).await.unwrap();
}

