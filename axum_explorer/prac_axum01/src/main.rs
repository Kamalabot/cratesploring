#![allow(warnings)]
#![allow(unused_imports)]

use axum;
use axum::handler::Handler;
use axum::response::Html;
use axum::routing::get;
use serde::Deserialize;
use tokio;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route(
            "/",
            get(|| async {
                let name = "prac1";
                format!("Hey you {}", name.clone())
            }),
        )
        .fallback(fallback)
        .route("/gh", get(ghtml))
        .route("/hr", get(html_read))
        .route("/st", get(show_st))
        .route("/pd", get(pic_d));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    format!("Route missing for: {}", uri)
}

async fn ghtml() -> Html<&'static str> {
    // axum::response::Html("<h1>Success Prac</h1>")
    "<h1>Success Prac</h1>".into()
}

async fn html_read() -> Html<&'static str> {
    let data = std::include_str!("../index.html");
    // axum::response::Html(data)
    data.into()
}

async fn show_st() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything works".to_owned())
}

async fn pic_d() -> impl axum::response::IntoResponse {
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
