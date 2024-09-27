#![allow(unused_imports)]

use anyhow::{Error as E, Ok, Result};
use candle_core::{DType, Device, Tensor};
use candle_examples::token_output_stream::TokenOutputStream;
use candle_transformers::models::gemma::Config;
use dotenvy::dotenv;
use hf_hub::{api::sync::ApiBuilder, Repo, RepoType};
use std::env;
use tokenizers::Tokenizer;

use std::io::Write;

// #[tokio::main]
fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let hf_token = std::env::var("HF_TOKEN").expect("Check where is .env file");

    println!("Got the token: {hf_token}");
    let api = ApiBuilder::new().with_token(Some(hf_token)).build()?;

    let model_id = "google/gemma-2b".to_string();
    let repo = api.repo(Repo::with_revision(
        model_id,
        RepoType::Model,
        "main".to_string(),
    ));
    let tokenizer_filename = repo.get("tokenizer.json")?;
    let tokenizer_file = Tokenizer::from_file(tokenizer_filename).map_err(anyhow::Error::msg)?;
    let mut tokenizer = TokenOutputStream::new(tokenizer_file);
    let prompt = "This is a simple text for tokenizing";
    // intialize the tokenizer
    let tokens = tokenizer
        .tokenizer()
        .encode(prompt, true)
        .map_err(E::msg)?
        .get_ids()
        .to_vec();
    // print the tokenised data
    println!("The length of the tokens:{}", tokens.len());
    for &t in tokens.iter() {
        if let Some(t) = tokenizer.next_token(t)? {
            println!("{t}")
        }
        println!("Raw value is :{t}")
    }
    std::io::stdout().flush()?;
    Ok(())
}
