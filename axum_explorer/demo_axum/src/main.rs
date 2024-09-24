#![allow(unused_imports)]
#![allow(warnings)]

use std::collections::HashMap;

use axum;
use axum::handler::Handler;
// above Handler trait required for fallback
use axum::extract::{Json, Query};
use axum::routing::{delete, get, patch, post, put};
use serde_json::{json, Value};
use tokio;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .fallback(fallback)
        .route("/", get(|| async { "Hey there" }))
        .route("/hello", get(hello))
        .route("/gethtml", get(get_html))
        .route("/sendhtml", get(send_html_file))
        .route("/dst", get(demo_status))
        .route("/duri", get(demo_uri))
        .route("/pd", get(pic_demo))
        .route(
            "/verb",
            get(got_verb)
                .put(put_verb)
                .patch(patch_verb)
                .post(post_verb)
                .delete(delete_verb),
        )
        .route("/path/:id", get(use_id))
        .route("/qp", get(use_qp))
        .route("/de", get(use_de))
        .route("/dj", get(use_js).post(post_js));

    let addr = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    axum::serve(addr, app).await.unwrap();
}

async fn hello() -> String {
    "Hellor Heror".into()
}

// fallback handler
async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No Route to heaven from: {}", uri),
    )
}

async fn get_html() -> axum::response::Html<&'static str> {
    "<h1>Hello World</h1>".into()
}

async fn send_html_file() -> axum::response::Html<&'static str> {
    std::include_str!("../index.html").into()
}

async fn demo_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is fine".into())
}

async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The uri is:{}", uri)
}

async fn pic_demo() -> impl axum::response::IntoResponse {
    use base64::Engine;
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
        base64::engine::general_purpose::STANDARD
            .decode(png)
            .unwrap(),
    )
}
//curl --request GET 127.0.0.1:3001/verb
async fn got_verb() -> String {
    "Get verb".to_string()
}
//curl --request POST 127.0.0.1:3001/verb
async fn post_verb() -> String {
    "Post verb".to_string()
}
//curl --request PUT 127.0.0.1:3001/verb
async fn put_verb() -> String {
    "Put verb".to_string()
}
//curl --request PATCH 127.0.0.1:3001/verb
async fn patch_verb() -> String {
    "Patch verb".to_string()
}
//curl --request DELETE 127.0.0.1:3001/verb
async fn delete_verb() -> String {
    "Delete verb".to_string()
}
//curl --request GET 127.0.0.1:3001/path/10
async fn use_id(axum::extract::Path(id): axum::extract::Path<i32>) -> String {
    format!("Recieved id is {}", id).into()
}
//curl --request GET 127.0.0.1:3001/qp/?item1=1\&item2=2
async fn use_qp(Query(data): Query<HashMap<String, String>>) -> String {
    format!("Recieved params are {:?}", data)
}

#[derive(serde::Deserialize)]
struct CustParams {
    name: String,
    age: i32,
}

impl std::fmt::Display for CustParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name is {} and age is {}", self.name, self.age)
    }
}
//curl --request GET 127.0.0.1:3001/cp?name=niagra\&age=25
async fn use_de(Query(cprm): Query<CustParams>) -> String {
    format!("Recieved custom Param: {}", cprm)
}

async fn use_js() -> Json<Value> {
    json!({ "name":"use json"}).into()
}
//curl \
// --request POST 'http://127.0.0.1:3001/dj' \
// --header "Content-Type: application/json" \
// --data '{"a":"b"}'
// note after -- there is no space
async fn post_js(Json(body): Json<Value>) -> String {
    // using axum extractor Json, and placing serde_json value
    format!("Extracting the body: {:?}", body)
}
