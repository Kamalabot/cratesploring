#![allow(unused_imports)]
#![allow(warnings)]

use anyhow::{Error as E, Ok, Result};
use candle_core::{Device, Tensor};
use candle_nn::{Linear, Module};
use candle_transformers::models::bert::{BertModel, Config};
use hf_hub::{Api, ApiBuilder, Repo, RepoType};
use ndarray::Array1;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use vector_embed::embed_prompt;

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

    Ok(())
}
