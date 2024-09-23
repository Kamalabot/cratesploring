#![allow(warnings)]
#![allow(unused_imports)]

mod book;
mod data;

use crate::book::Book;
use crate::data::DATA;
// need to use thread to access the data
use std::thread;

use axum::handler::Handler;
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use axum::extract::{Form, Json, Path, Query};
use serde_json::{json, Value};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn print_data() {
    // print the data by accessing it mutably
    // and in thread safe manner
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("data: {:?}", data);
    })
    .join()
    .unwrap();
}

async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    format!("No route to: {}", uri)
}

async fn get_books() -> axum::response::Html<String> {
    let handle = thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| a.title.cmp(&b.title));
        books
            .iter()
            .map(|&b| format!("<p>{}</p>\n", &b))
            .collect::<String>()
    });
    handle.join().unwrap().into()
}

async fn get_books_id(Path(id): Path<u32>) -> axum::response::Html<String> {
    let hnd = thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!("<p>{}</p>\n", &book),
            None => format!("<p>Book id: {} not found</p>", id),
        }
    });
    hnd.join().unwrap().into()
}

async fn put_books(Json(b): Json<Book>) -> axum::response::Html<String> {
    // println!("entering put books");
    let hnd = thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        data.insert(b.id, b.clone());
        format!("Placed book: {}", &b)
    });
    hnd.join().unwrap().into()
}

async fn get_books_id_form(Path(id): Path<u32>) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(b) => format!(
                concat!(
                    "<form method=\"post\" action=\"/gb/{}/form\">\n",
                    "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                    "<p><input name=\"author\" value=\"{}\"></p>\n",
                    "<p><input name=\"title\" value=\"{}\"></p>\n",
                    "<input type=\"submit\" value=\"Save\">\n"
                ),
                &b.id, &b.id, &b.author, &b.title
            ),
            None => format!("<p>Book id {} not found</p>", id),
        }
    })
    .join()
    .unwrap()
    .into()
}

async fn post_book_id_form(form: Form<Book>) -> axum::response::Html<String> {
    let new_book: Book = form.0;
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&new_book.id) {
            data.insert(new_book.id, new_book.clone());
            format!("Posted book: {}", &new_book.title)
        } else {
            format!("Book id not found: {}", &new_book.title)
        }
    })
    .join()
    .unwrap()
    .into()
}

// curl \
// --request POST 'localhost:3000/books/1/form' \
// --header "Content-Type: application/x-www-form-urlencoded" \
// --data "id=1"  \
// --data "title=Another Title" \
// --data "author=Someone Else"

async fn delete_book(Path(id): Path<u32>) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&id) {
            data.remove(&id);
            format!("Data removed")
        } else {
            format!("Data not found")
        }
    })
    .join()
    .unwrap()
    .into()
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    let host = [127, 0, 0, 1];
    let port = 2999;
    let addr = SocketAddr::from((host, port));
    // print_data().await;
    let app = Router::new()
        .route("/gb", get(get_books).put(put_books))
        .route("/gb/:id", get(get_books_id).delete(delete_book))
        .route(
            "/gb/:id/form",
            get(get_books_id_form).post(post_book_id_form),
        )
        .fallback(fallback);
    // let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
