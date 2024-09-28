#![allow(unused_imports)]
#![allow(warnings)]

use axum::{
    extract::{Json, Path, Query},
    serve,
};
use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use prac_axum03::axum_funcs::*;
use serde_json::{json, Value};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "4th practice of Axum" }))
        .route("/:id", get(path_finder))
        .route("/prac/:pid", get(async_show_prac))
        .route("/updt/:pid", patch(async_uncomplete))
        .route("/delete/:pid", delete(async_remove_sess))
        .route("/prac", post(async_create_prac))
        .route("/show", get(async_show_all));
    let host = [127, 0, 0, 1];
    let port = 3030;
    let addr = SocketAddr::from((host, port));
    let lstr = TcpListener::bind(addr).await.unwrap();
    serve(lstr, app).await.unwrap()
}
