#![allow(unused_imports)]
#![allow(warnings)]

use surrealdb::{Surreal};
use candle_core::{Tensor, Device};
use candle_nn::{Module, Linear};
use ndarray::Array1;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use anyhow::{Error as E, Ok, Result};
use vector_embed::{Embeddings, Params, Model};
use hf_hum::{Repo, RepoType, Api, ApiBuilder};
use candle_transformers::models::bert::{BertModel, Config};

   
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    client.signin(Root{
        username:"root",
        password:"root"
    }).await?;
    client.use_ns("test_ns").use_db("test_db").await?;

    dotenv().ok();
    let hf_token = std::env::var("HF_TOKEN").expect("Check where is .env file");

    println!("Got the token: {hf_token}");
    let api = ApiBuilder::new().with_token(Some(hf_token)).build()?;

    let model_id = "sentence-transformers/all-MiniLM-L6-v2".to_string();
    let repo = api.repo(Repo::with_revision(
        model_id,
        RepoType::Model,
        "main".to_string(),
    ));
    // setting up the devices
    let device = Device::new_cuda(0)?;
    let dtype = DType::F32;
    // let device = Device::Cpu;
    // let dtype = DType::BF16; // unsupported for op matmul
    // let dtype = DType::F16;
    // below code brings the tokenizer to local machine, load & use 
    let start = std::time::Instant::now();
    let tokenizer_filename = repo.get("tokenizer.json")?;
    let tokenizer_file = Tokenizer::from_file(tokenizer_filename.clone()).map_err(anyhow::Error::msg)?;
    let mut tokenizer = TokenOutputStream::new(tokenizer_file);
    // below code will bring the model.safetensors first
    let config_filename = repo.get("config.json")?;
    let config: Config = serde_json::from_reader(std::fs::File::open(config_filename)?)?;
    let config_vec = config.to_vec();
    // get the model.safetensors
 //    let mdl_idx_json_file = candle_examples::hub_load_safetensors(&repo, "modules.json")?;
 // 
 //    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&mdl_idx_json_file, dtype, &device)?};
 //    // turning flash_attn to false
 //    println!("loaded the tensors in memory {:?}", start.elapsed());
 //    // loading the model into the pipeline
 //    let mut getmodel = Model::load(weights, tokenizer, config)?;
    println!("Downloaded the files {:?}", start.elapsed());
    Ok(())
}
