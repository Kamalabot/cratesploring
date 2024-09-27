#![allow(unused_variables)]
#![allow(warnings)]

use dotenvy::dotenv;
use hf_hub::api::sync::ApiBuilder;
use std::env;

#[tokio::main]
async fn main() {
    // need to have .env file in the same folder
    // where the executable is run
    dotenv().ok(); // get the hf token
    let hf_token = env::var("HF_TOKEN").expect("HF_TOKEN not set");
    println!("Got the token: {hf_token}");
    let api = ApiBuilder::new()
        .with_token(Some(hf_token))
        .build()
        .unwrap();
    let model_id = "google/gemma-2-2b".to_owned();
    let file_name = "config.json";
    let config_dload = api.model(model_id).get(file_name).unwrap();
    // file gets downloaded to
    // ~/.cache/huggingface/hub/models--google--gemma-2-2b/snapshots/c5ebcd40d208330abc697524c919956e692655cf/config.json
    println!("Hello, world!");
}
