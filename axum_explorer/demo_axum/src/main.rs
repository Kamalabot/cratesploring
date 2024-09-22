#![allow(unused_imports)]
#![allow(warnings)]

use axum;
use axum::handler::Handler;
// above Handler trait required for fallback
use axum::routing::get;
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
        .route("/pd", get(pic_demo));

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
