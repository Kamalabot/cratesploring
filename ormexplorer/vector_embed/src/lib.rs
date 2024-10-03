#![allow(warnings)]
// the following function is already explained
// in the other video introducing embedding models
// so here will just go through the overview...
// We see the BertModel & its rust native config is imported
use candle_transformers::models::bert::{BertModel, Config, HiddenAct, DTYPE};

use anyhow::{Error as E, Ok, Result};
// Candle_core and candle_nn crates are used for building the model
use candle_core::{Device, Tensor};
use candle_nn::VarBuilder;
// clap is the command line parser, we are deriving the Parser from it
// hf_hub api provides the interface to download the models
use hf_hub::{api::sync::Api, Repo, RepoType};
// tokenizer to split the sentence & encode it to numbers
use tokenizers::{PaddingParams, Tokenizer};
// below build_model_and_tokenizer function pulls the
// model, and loads into rust execution environment...
pub fn build_model_and_tokenizer() -> Result<(BertModel, Tokenizer)> {
    // setting the device
    let device = Device::Cpu;
    // currently this function supports only BertModel arch based embedding models
    let default_model = "sentence-transformers/all-MiniLM-L6-v2".to_string();
    let default_revision = "refs/pr/21".to_string();

    // Below the repo instances are creaated on the model_id provided
    let repo = Repo::with_revision(default_model, RepoType::Model, default_revision);
    // following lines download the files
    let (config_filename, tokenizer_filename, weights_filename) = {
        let api = Api::new()?;
        let api = api.repo(repo);
        // download config.json
        let config = api.get("config.json")?;
        // get tokenizer.json
        let tokenizer = api.get("tokenizer.json")?;
        // get the actual model bin / safetensors file
        let weights = api.get("model.safetensors")?;
        (config, tokenizer, weights)
    };
    // Here the model building starts inside rust
    let config = std::fs::read_to_string(config_filename)?;
    let mut config: Config = serde_json::from_str(&config)?;
    // tokenizer is built
    let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;
    // model is built with the config.json and the downloaded weights
    // config.json contains the necessary settings, that will intialize the Bert Model
    // inside rust.. Using the above import.
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)? };
    // the activation method in the embedding is modified below
    config.hidden_act = HiddenAct::GeluApproximate;
    let model = BertModel::load(vb, &config)?;
    Ok((model, tokenizer))
    // model and tokenizers are returned
}
// The main function starts here
pub fn embed_prompt(prompt: &str) -> Result<Tensor> {
    // timing the execution using start::Instant
    let start = std::time::Instant::now();
    // calling the build_model_and_tokenizer function and getting the model & tokenizer
    let (model, mut tokenizer) = build_model_and_tokenizer()?;
    // setting the device on which the model is loaded
    let device = &model.device;
    // In this example the model is loaded, and embedding is generated very fast
    // We will see the RAM usage next, As we saw how ram got used and model got released
    // Tokenizer is setup
    let tokenizer = tokenizer
        .with_padding(None)
        .with_truncation(None)
        .map_err(E::msg)?;
    // The prompt is tokenized below...
    let tokens = tokenizer
        .encode(prompt, true)
        .map_err(E::msg)?
        .get_ids()
        .to_vec();
    // token ids are made ready for embedding
    let token_ids = Tensor::new(&tokens[..], device)?.unsqueeze(0)?;
    let token_type_ids = token_ids.zeros_like()?;
    println!("Loaded and encoded {:?}", start.elapsed());
    // below the embedding process is done using the forward function
    let start = std::time::Instant::now();
    let ys = model.forward(&token_ids, &token_type_ids, None)?;
    println!("{ys}");
    println!("Took {:?}", start.elapsed());
    Ok(ys)
    // The code walkthrough of part 1 is done...
    // The embedding vectors are returned...
}

pub fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}
