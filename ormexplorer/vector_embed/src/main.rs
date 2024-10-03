#![allow(unused_imports)]
#![allow(warnings)]
// Before working on the imports... Lets look at the
// Cargo.toml
// Much of the modules and their imports will be clear now...
use anyhow::{Error as E, Ok, Result};
use candle_core::Tensor;
use serde::{Deserialize, Serialize};
use std::io::stdin;
// Following imports provide access to the SurrealDB
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal; // this provides the connectivity
use vector_embed::embed_prompt; // We will look it after this walkthrough...

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}
// Above Record is required when querying SurrealDB

// The vector is represented as an array of floating-point numbers.
// Below is the struct that stores the text and the vector embedding
#[derive(Debug, Deserialize, Serialize)]
struct Document {
    text: String,
    emb_vector: Vec<f32>, // Tensors cannot be Serialize / Deserialize so need
                          // to use vectors only...
}
// Main function starts...
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Database is Connection is established...
    let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    // the database conn is authenticated....
    client
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await?;
    // Namespace and Database is declared and connecetd...
    // After this, rust is having the same access that we had
    // in the SQL terminal...
    client.use_ns("test_ns").use_db("test_db").await?;

    println!("Connection established...");
    // lets just create data, and store vectors
    // User input is solicited...
    let mut in_str = String::new();
    stdin().read_line(&mut in_str)?;
    // embed_prompt function is imported from the lib.rs
    let embed = embed_prompt(&in_str)?;
    // The embedding is returned as 3D tensors...
    // Apply some avg-pooling by taking the mean embedding value for all tokens (including padding)
    let (_n_sentence, n_tokens, _hidden_size) = embed.dims3()?;
    // starting to calculate the distances between embeddings of the sentences
    let embeddings = (embed.sum(1)? / (n_tokens as f64))?;
    // normalize_embeddings to avoid -ve or outliers numbers
    let embeddings = normalize_l2(&embeddings)?;
    // the above steps make the embedding into 1d Tensor...

    println!("Pooled Embedding of {in_str} is {:?}", embeddings);

    println!("Storing the embedding into db after converting it to Vec<f32>");
    // Following code converts Tensor to Vec<f32>, for storing
    let emb_vector: Vec<f32> = embeddings.get(0)?.to_vec1().unwrap();
    // following line creates the document instance
    // and stores into the database...
    let _e: Option<Record> = client
        .create("vstore") // vstore iis the table name...
        .content(Document {
            text: in_str,
            emb_vector,
        })
        .await?;
    println!("Data written.. Check by SELECT");
    // next we use the select function to get all the data in the table
    let docs: Vec<Document> = client.select("vstore").await?;
    // following lines print the output... including the embedding vector...
    println!("Number of docs in the Vstore: {}", docs.len());
    for doc in docs {
        println!("{:?}", doc)
    }
    Ok(())
}
// embedding is normalized to avoid large or negative numbers...
pub fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    // Divide each embedding vector by its L2 norm (square root of the sum of squared values).
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}
