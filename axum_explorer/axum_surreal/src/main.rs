#![allow(warnings)]
#![allow(unused_imports)]
use axum::extract::State;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

use axum_surreal::{conn_sdb, create_employee, get_employee, get_employees, AppState, Employee};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let db = conn_sdb().await;

    let app_state = AppState { db: Arc::new(db) };

    let app = Router::new()
        .route("/employees", post(create_employee))
        .route("/", get(|| async { "Server is up" }))
        .route("/employee/:id", get(get_employee))
        .route("/allemps", get(get_employees))
        .with_state(app_state);

    let lsnr = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(lsnr, app).await.unwrap();
}
