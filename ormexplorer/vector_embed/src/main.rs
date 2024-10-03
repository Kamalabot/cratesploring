#![allow(unused_imports)]
#![allow(warnings)]

use anyhow::{Error as E, Ok, Result};
use candle_core::Tensor;
use serde::{Deserialize, Serialize};
use std::io::stdin;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use vector_embed::embed_prompt;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
// The vector is represented as an array of floating-point numbers.
#[derive(Debug, Deserialize, Serialize)]
struct Document {
    text: String,
    emb_vector: Vec<f32>,
}
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    client
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
    client.use_ns("test_ns").use_db("test_db").await?;
    println!("Connection established...");
    // lets just create data, and store vectors
    let mut in_str = String::new();
    stdin().read_line(&mut in_str)?;
    let embed = embed_prompt(&in_str)?;
    // Apply some avg-pooling by taking the mean embedding value for all tokens (including padding)
    let (_n_sentence, n_tokens, _hidden_size) = embed.dims3()?;
    // starting to calculate the distances between embeddings of the sentences
    let embeddings = (embed.sum(1)? / (n_tokens as f64))?;
    // normalize_embeddings to avoid -ve or outliers numbers
    let embeddings = normalize_l2(&embeddings)?;

    println!("Pooled Embedding of {in_str} is {:?}", embeddings);

    println!("Storing the embedding into db after converting it to Vec<f32>");

    let emb_vector: Vec<f32> = embeddings.get(0)?.to_vec1().unwrap();

    let _e: Option<Record> = client
        .create("vstore")
        .content(Document {
            text: in_str,
            emb_vector,
        })
        .await?;
    println!("Data written.. Check by SELECT");
    let docs: Vec<Document> = client.select("vstore").await?;

    println!("Number of docs in the Vstore: {}", docs.len());
    for doc in docs {
        println!("{:?}", doc)
    }
    Ok(())
}

pub fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    // Divide each embedding vector by its L2 norm (square root of the sum of squared values).
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}
