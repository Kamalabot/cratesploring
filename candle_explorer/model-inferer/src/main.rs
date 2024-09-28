#![allow(unused_imports)]

use anyhow::{Error as E, Ok, Result};
use candle_core::{DType, Device, Tensor};

use candle_examples::token_output_stream::TokenOutputStream;
use candle_transformers::models::gemma2::Config;
use candle_transformers::models::gemma2::Model;
use candle_transformers::generation::LogitsProcessor;
use candle_nn::VarBuilder;

use dotenvy::dotenv;
use hf_hub::{api::sync::ApiBuilder, Repo, RepoType};
use std::env;

use tokenizers::Tokenizer;

use std::io::Write;

struct TextGeneration {
    model: Model,
    device: Device,
    tokenizer: TokenOutputStream,
    logits_processor: LogitsProcessor,
    repeat_penalty: f32,
    repeat_last_n: usize,
}

impl TextGeneration {
    #[allow(clippy::too_many_arguments)]
    fn new(
        model: Model,
        tokenizer: Tokenizer,
        seed: u64,
        temp: Option<f64>,
        top_p: Option<f64>,
        repeat_penalty: f32,
        repeat_last_n: usize,
        device: &Device,
    ) -> Self {
        let logits_processor = LogitsProcessor::new(seed, temp, top_p);
        Self {
            model,
            tokenizer: TokenOutputStream::new(tokenizer),
            logits_processor,
            repeat_penalty,
            repeat_last_n,
            device: device.clone(),
        }
    }

    fn run(&mut self, prompt: &str, sample_len: usize) -> Result<()> {
        use std::io::Write;
        self.tokenizer.clear();
        let mut tokens = self
            .tokenizer
            .tokenizer()
            .encode(prompt, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();
        for &t in tokens.iter() {
            if let Some(t) = self.tokenizer.next_token(t)? {
                print!("{t}")
            }
        }
        std::io::stdout().flush()?;

        let mut generated_tokens = 0usize;
        let eos_token = match self.tokenizer.get_token("<eos>") {
            Some(token) => token,
            None => anyhow::bail!("cannot find the <eos> token"),
        };
        let start_gen = std::time::Instant::now();
        for index in 0..sample_len {
            let context_size = if index > 0 { 1 } else { tokens.len() };
            let start_pos = tokens.len().saturating_sub(context_size);
            let ctxt = &tokens[start_pos..];
            let input = Tensor::new(ctxt, &self.device)?.unsqueeze(0)?;
            let logits = self.model.forward(&input, start_pos)?;
            let logits = logits.squeeze(0)?.squeeze(0)?.to_dtype(DType::F32)?;
            let logits = if self.repeat_penalty == 1. {
                logits
            } else {
                let start_at = tokens.len().saturating_sub(self.repeat_last_n);
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    self.repeat_penalty,
                    &tokens[start_at..],
                )?
            };

            let next_token = self.logits_processor.sample(&logits)?;
            tokens.push(next_token);
            generated_tokens += 1;
            if next_token == eos_token {
                break;
            }
            if let Some(t) = self.tokenizer.next_token(next_token)? {
                print!("{t}");
                std::io::stdout().flush()?;
            }
        }
        let dt = start_gen.elapsed();
        if let Some(rest) = self.tokenizer.decode_rest().map_err(E::msg)? {
            print!("{rest}");
        }
        std::io::stdout().flush()?;
        println!(
            "\n{generated_tokens} tokens generated ({:.2} token/s)",
            generated_tokens as f64 / dt.as_secs_f64(),
        );
        Ok(())
    }
}   
// #[tokio::main]
fn main() -> Result<(), anyhow::Error> {
    dotenv().ok();
    let hf_token = std::env::var("HF_TOKEN").expect("Check where is .env file");

    println!("Got the token: {hf_token}");
    let api = ApiBuilder::new().with_token(Some(hf_token)).build()?;

    let model_id = "google/gemma-2-2b".to_string();
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
    let prompt = "Where is the sun located in the Universe";
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
    // below code will bring the model.safetensors first
    let config_filename = repo.get("config.json")?;
    // get the model.safetensors.index.json
    let mdl_idx_json_file = candle_examples::hub_load_safetensors(&repo, "model.safetensors.index.json")?;
    println!("retrieved the files in {:?}", start.elapsed());
    // building varbuilder
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&mdl_idx_json_file, dtype, &device)?};
    let config: Config = serde_json::from_reader(std::fs::File::open(config_filename)?)?;
    let model = Model::new(false, &config, vb)?;
    // turning flash_attn to false
    println!("loaded the tensors in memory {:?}", start.elapsed());
    // loading the model into the pipeline
    let tokenizer_file = Tokenizer::from_file(tokenizer_filename).map_err(anyhow::Error::msg)?;
    let prompt = "Where is the sun located in the Universe";
    let mut pipeline = TextGeneration::new(
        model,
        tokenizer_file,
        299792486,
        Some(0.5),
        Some(0.7),
        1.1,
        64,
        &device,
    );
    pipeline.run(prompt, 100)?;   
    println!("Completed inference in {:?}", start.elapsed());
    Ok(())
}
