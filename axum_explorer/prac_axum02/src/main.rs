#![allow(warnings)]
#![allow(unused_imports)]
mod fncall;

use crate::fncall::*;
use axum::extract::{Json, Path, Query};
use axum::{handler::Handler, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::net::SocketAddr;
use tokio;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "hey there" }))
        .route("/rj", get(ret_json).post(js_handle))
        .route("/:id", get(path_handle))
        .route("/qp", get(qp_handle))
        .route("/pa", get(field_param))
        .route("/qs", get(qp_str))
        .route("/fncall", get(get_func_body));
    let host = [127, 0, 0, 1];
    let port = 3000;
    let addr = SocketAddr::from((host, port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize, Debug, Serialize)]
struct Parser {
    name: String,
    age: i32,
}

async fn qp_handle(Query(parm): Query<HashMap<String, String>>) -> String {
    format!("{:?}", parm).into()
}

async fn field_param(Query(fields): Query<Parser>) -> String {
    let js_val = serde_json::to_value(&fields).unwrap();
    format!("{}", js_val)
    // format!("{:?}", fields).into()
}

async fn js_handle(Json(dict): Json<Value>) -> String {
    format!("{}", dict)
}

async fn ret_json() -> Json<Value> {
    json!({"a":"b"}).into()
}

async fn path_handle(Path(id): Path<i32>) -> Json<Value> {
    json!({"id": id}).into()
}

// async fn qp_str(Query(s): Query<String>) -> String {
async fn qp_str(Query(s): Query<Value>) -> String {
    format!("{}", s)
}
