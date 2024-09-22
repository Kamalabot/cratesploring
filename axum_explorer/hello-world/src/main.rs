#![allow(warnings)]
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router, ServiceExt,
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

// https://github.com/tokio-rs/axum/tree/v0.7.x
//
#[tokio::main]
async fn main() {
    // Route all req on / to anonymous handler
    // handler is async func
    // closure can be a handler

    let app = Router::new()
        .route("/", get(handler))
        .route("/users", post(create_user));

    // share the address for server to bind
    let addr = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    // following will resolve, as per GPT
    axum::serve(addr, app).await.unwrap();
}

async fn handler() -> &'static str {
    "hello there"
}

async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1225,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

// curl -X POST http://127.0.0.1:3000/users \
// -H "Content-Type: application/json" \
// -d '{"username": "new123"}'
