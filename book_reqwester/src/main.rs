#![allow(warnings)]
#![allow(unused_imports)]
mod book;
use crate::book::Book;
use std::{error::Error, fmt::format};

use reqwest::{get, Client};
use tokio;

use serde_json::{from_str, json, to_string};

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // book_req().await?;
    let client = Client::new();
    // base url response
    let base = "http://127.0.0.1:3001/";
    let base_resp = client.get(base).send().await?.text().await?;

    println!("Response form hello handler: {}", base_resp);

    let get_eps = vec!["hello", "gethtml", "sendhtml", "dst", "duri", "fb"];
    // Getting multi responses
    println!("Starting multi endpoint test");

    for elm in get_eps.iter() {
        println!("Sending request on {elm} endpoint");
        let turl = format!("{}{}", base, elm);
        let tresp = client.get(turl).send().await?.text().await?;
        println!("response from {elm} endpoint: {tresp}");
    }
    let verb_url = format!("{}verb", base);
    let verb_get_resp = client.get(&verb_url).send().await?.text().await?;
    let verb_put_resp = client.put(&verb_url).send().await?.text().await?;
    let verb_post_resp = client.post(&verb_url).send().await?.text().await?;
    let verb_patch_resp = client.patch(&verb_url).send().await?.text().await?;
    let verb_del_resp = client.delete(&verb_url).send().await?.text().await?;
    println!("Response form get req: {}", verb_get_resp);
    println!("Response form put req: {}", verb_put_resp);
    println!("Response form post req: {}", verb_post_resp);
    println!("Response form patch req: {}", verb_patch_resp);
    println!("Response form delete req: {}", verb_del_resp);

    let id_url = format!("{base}path/{id}", id = 6);
    let id_resp = client.get(id_url).send().await?.text().await?;
    println!("The id_url response is: {id_resp}");

    let qp_url = format!("{base}qp?name=noname&height=86");
    let qp_resp = client.get(qp_url).send().await?.text().await?;
    println!("The qp_url response is: {qp_resp}");
    // the query params have to be as per the struct name and dtypes
    let de_url = format!("{base}de?name=noname&age=86");
    let de_resp = client.get(de_url).send().await?.text().await?;
    println!("The de_url response is: {de_resp}");

    let de_url_w = format!("{base}de?name=noname&weight=86");
    let de_resp = client.get(de_url_w).send().await?.text().await?;
    // will throw failed to deserialize error, as age is missing
    println!("The de_url response is: {de_resp}");

    let js_url = format!("{base}dj");
    let js_res = client.get(js_url).send().await?.text().await?;
    // the output from the server is text, so it prints without issue
    println!("Js output: {}", js_res);
    let json_data = json!({"data":"entry"});
    let post_js_url = format!("{base}dj");
    let post_res = client
        .post(post_js_url)
        .json(&json_data)
        .send()
        .await?
        .text()
        .await?;
    println!("Post output: {}", post_res);
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
