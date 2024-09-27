use anyhow::{anyhow, Error as E, Result};
use candle_core::{Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use hf_hub::{api::sync::Api, Cache, Repo, RepoType};
use tokenizers::Tokenizer; // this is external
                           // importing the model Structs
use candle_tutorial::models::roberta::{RobertaConfig, RobertaModel, FLOATING_DTYPE};

fn build_model_and_tokenizer() -> Result<(RobertaModel, Tokenizer)> {
    // returns the model and tokenizer
    let device = Device::Cpu;
    let default_model = "roberta-base".to_string();
    let default_revision = "main".to_string();
    let (model_id, revision) = (default_model, default_revision);
    let repo = Repo::with_revision(model_id, RepoType::Model, revision);
    let offline = false;

    let (config_filename, tokenizer_filename, weights_filename) = if offline {
        // this part is loading the files from hf-folder in cache
        let cache = Cache::default().repo(repo);
        (
            cache
                .get("config.json")
                .ok_or(anyhow!("Missing config file in cache"))?,
            cache
                .get("tokenizer.json")
                .ok_or(anyhow!("Missing tokenizer file in cache"))?,
            cache
                .get("model.safetensors")
                .ok_or(anyhow!("Missing weights file in cache"))?,
        )
    } else {
        let api = Api::new()?;
        let api = api.repo(repo);
        println!("Getting the model safetensor from hf");
        (
            api.get("config.json")?,
            api.get("tokenizer.json")?,
            api.get("model.safetensors")?,
        )
    };

    println!("config_filename: {}", config_filename.display());

    let config = std::fs::read_to_string(config_filename)?;
    let config: RobertaConfig = serde_json::from_str(&config)?;
    // loading the tokenizer using Tokenizer struct
    let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;

    let vb = unsafe {
        VarBuilder::from_mmaped_safetensors(&[weights_filename], FLOATING_DTYPE, &device)?
    };
    println!("Loading the model into code");
    let model = RobertaModel::load(vb, &config)?;
    Ok((model, tokenizer))
}

fn main() -> Result<()> {
    let (model, _tokenizer) = build_model_and_tokenizer()?;
    // moving model to device
    let device = &model.device;
    // making a simple input_id array
    let input_ids = &[
        [0u32, 31414, 232, 328, 740, 1140, 12695, 69, 46078, 1588, 2],
        [0u32, 31414, 232, 328, 740, 1140, 12695, 69, 46078, 1588, 2],
    ];
    // casting it to tensor
    let input_ids = Tensor::new(input_ids, &device)?;

    let token_ids = input_ids.zeros_like()?;

    println!("token_ids: {:?}", token_ids.to_vec2::<u32>()?);
    // token_ids seems like atten masks
    println!("input_ids: {:?}", input_ids.to_vec2::<u32>()?);
    // execute the logits generation
    let output = model.forward(&input_ids, &token_ids)?;
    // let output = output.squeeze(0)?;

    println!("output: {:?}", output.i((.., 0))?.dims2());

    let logits = &[[0.1_f32, 0.2], [0.5, 0.6]];
    let logits = Tensor::new(logits, &device)?;

    println!("logits: {:?}", logits.i((.., 0))?.to_vec1::<f32>()?);
    println!("logits: {:?}", logits.i((.., 1))?.to_vec1::<f32>()?);
    // the output is not converted to text!!!

    Ok(())
}
