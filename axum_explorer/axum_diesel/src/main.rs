#![allow(unused_imports)]
#![allow(warnings)]

use axum::extract::{Json, Path, Query};
use axum::handler::Handler;
use axum::routing::{delete, get, patch, post};
use axum_diesel::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = axum::Router::new()
        .route("/", get(|| async { "Axum Server" }))
        .route("/all", get(show_employees))
        .route("/show/:sid", get(show_employee))
        .route("/rm/:pid", delete(delete_employee))
        .route("/make", post(create_employee))
        // following is for allowing access inside same machine servers
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    println!("Hello, world!");
    Ok(())
}
