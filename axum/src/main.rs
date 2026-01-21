use std::sync::Arc;

use axum::{Router, extract::State, routing::{get, post}, serve};
use tokio::{net::TcpListener, sync::RwLock};

type Collection = Vec<String>;

#[tokio::main]
async fn main() {
    let db = Arc::new(RwLock::new(Collection::new()));

    let app = Router::new()
        .route("/", get(handler))
        .route("/ping", get(|| async { "pong" }))
        .route("/todos", post(add_todo).get(list_todos))
        .with_state(db);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    _ = serve(listener, app).await;
}

async fn handler() -> &'static str {
    "Hello, World!"
}

async fn list_todos(State(db): State<Arc<RwLock<Collection>>>) -> &'static str {
    let todos = db.read().await;
    if todos.is_empty() {
        "No todos found."
    } else {
        "Listing todos..."
    }
}

async fn add_todo(State(db): State<Arc<RwLock<Collection>>>, todo: String) -> &'static str {
    let mut todos = db.write().await;
    todos.push(todo);
    "Todo added."
}