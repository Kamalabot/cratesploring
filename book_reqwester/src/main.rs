#![allow(warnings)]
#![allow(unused_imports)]
mod book;
use crate::book::Book;
use std::error::Error;

use reqwest::{get, Client};
use tokio;

use serde_json::{from_str, to_string};

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // book_req().await?;
    let client = Client::new();
    let base = "http://127.0.0.1:3001/";
    let hello_resp = client.get(base).send().await?.text().await?;
    println!("Response form hello handler: {}", hello_resp);
    Ok(())
}

async fn book_req() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    // client builds the request, and sends it with send()
    let get_books_resp = client.get("http://127.0.0.1:2999/gb").send().await?;
    // explore the parts of the response
    println!("Status is {}", get_books_resp.status());
    println!("Headers are {:?}", get_books_resp.headers());

    println!("get_books request: {}", get_books_resp.text().await?);

    let get_b_id_resp = client.get("http://127.0.0.1:2999/gb/1").send().await?;
    println!("get_books_id request: {}", get_b_id_resp.text().await?);

    let put_b1 = Book {
        id: 4,
        title: "Booked".to_string(),
        author: "Wroteme".to_string(),
    };

    let put_b1_string = to_string(&put_b1).unwrap();

    // tested the string aboge, thinking of sending it
    println!("string from book: {}", put_b1_string);

    let put_b_id_resp = client
        .put("http://127.0.0.1:2999/gb")
        // .json(&put_b1_string)
        .json(&put_b1) // serialize struct to json
        .send()
        .await?
        .text()
        .await?;

    println!("put_b_id_resp response: {}", put_b_id_resp);
    Ok(())
}
