#![allow(warnings)]

use axum::{extract::Query, response::Html, routing::get, Router};
use rand::{thread_rng, Rng};
use serde::Deserialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/html", get(index));

    let addr = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    axum::serve(addr, app).await.unwrap();
}

#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

//http://127.0.0.1:3003/?start=50&end=100

async fn handler(Query(range): Query<RangeParameters>) -> Html<String> {
    let rand_num = thread_rng().gen_range(range.start..range.end);
    Html(format!("<h1>Random Number: {}</h1>", rand_num))
}

async fn index() -> Html<&'static str> {
    Html(std::include_str!("../index.html"))
}
