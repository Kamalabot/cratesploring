// Flags to avoid unwanted warnings
#![allow(unused_imports)]
// Imports required
use anyhow::{Error as E, Ok, Result};
use candle_core::{DType, Device, Tensor};

use candle_examples::token_output_stream::TokenOutputStream;
use candle_transformers::models::gemma2::Config;
use candle_transformers::models::gemma2::Model;
// Model that is built below is imported from candle_transformers crate
use candle_transformers::generation::LogitsProcessor;
use candle_nn::VarBuilder;
// dotenv for loading the .env file
use dotenvy::dotenv;
use hf_hub::{api::sync::ApiBuilder, Repo, RepoType};
use std::env;
// Tokenizer to get the tokenized output
use tokenizers::Tokenizer;

use std::io::Write;
// I have pulled this code from candle_transformers to here
// for showing.. How TextGeneration works
//
struct TextGeneration {
    model: Model,
    device: Device,
    tokenizer: TokenOutputStream,
    logits_processor: LogitsProcessor,
    repeat_penalty: f32,
    repeat_last_n: usize,
} // above parameters are self-explanotory, or a google search will 
// provide the details.

impl TextGeneration {
    // following code will instantiate a TextGeneration Object
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
        // This LogitsProcessor contains the Tensors of the Gemma Model
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
    // here is the run function implementation of the pipeline 
    fn run(&mut self, prompt: &str, sample_len: usize) -> Result<()> {
        use std::io::Write;
        self.tokenizer.clear();
        // tokenising the given prompt
        let mut tokens = self
            .tokenizer
            .tokenizer()
            .encode(prompt, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();
        // printing the next tokens
        for &t in tokens.iter() {
            if let Some(t) = self.tokenizer.next_token(t)? {
                print!("{t}")
            }
        }
        std::io::stdout().flush()?;
        // Initially generated tokens are 0
        let mut generated_tokens = 0usize;
        // End of Statement token is identified
        let eos_token = match self.tokenizer.get_token("<eos>") {
            Some(token) => token,
            None => anyhow::bail!("cannot find the <eos> token"),
        };
        // generating the tokens start here
        let start_gen = std::time::Instant::now();
        // For loop below does the generation, and 
        // creates sample_len of tokens
        for index in 0..sample_len {
            let context_size = if index > 0 { 1 } else { tokens.len() };
            let start_pos = tokens.len().saturating_sub(context_size);
            let ctxt = &tokens[start_pos..];
            let input = Tensor::new(ctxt, &self.device)?.unsqueeze(0)?;
            // here is the model.forward() where the next tokens are predicted
            let logits = self.model.forward(&input, start_pos)?;
            let logits = logits.squeeze(0)?.squeeze(0)?.to_dtype(DType::F32)?;
            let logits = if self.repeat_penalty == 1. {
                logits
            } else {
                let start_at = tokens.len().saturating_sub(self.repeat_last_n);
                // this loo checks if the tokens generated are repeated
                // and penalty is applied
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    self.repeat_penalty,
                    &tokens[start_at..],
                )?
            };
            // Here the sample of generated tokens are done
            let next_token = self.logits_processor.sample(&logits)?;
            tokens.push(next_token);
            // and that token is pushed into output tokens vector
            generated_tokens += 1;
            if next_token == eos_token {
                // eos token will break the loop
                break;
            }
            if let Some(t) = self.tokenizer.next_token(next_token)? {
                // if there is next token then print it
                print!("{t}");
                std::io::stdout().flush()?;
            }
        }
        // Generation is completed
        let dt = start_gen.elapsed();
        if let Some(rest) = self.tokenizer.decode_rest().map_err(E::msg)? {
            print!("{rest}");
        }
        std::io::stdout().flush()?;
        // The time is printed
        println!(
            "\n{generated_tokens} tokens generated ({:.2} token/s)",
            generated_tokens as f64 / dt.as_secs_f64(),
        );
        Ok(())
    }
}   
//Main starts here...
// #[tokio::main]
fn main() -> Result<(), anyhow::Error> {
    // to download model file we use hf_api
    dotenv().ok();
    // .env file is loaded into environment variables
    let hf_token = std::env::var("HF_TOKEN").expect("Check where is .env file");
    // HF_TOKEN env_var is read by the code

    println!("Got the token: {hf_token}");
    let api = ApiBuilder::new().with_token(Some(hf_token)).build()?;
    // the ApiBuilder has to be used with token in the header..

    let model_id = "google/gemma-2-2b".to_string();
    // Model id is that is used 
    let repo = api.repo(Repo::with_revision(
        model_id,
        RepoType::Model,
        "main".to_string(),
    )); // Innstance of the repo is created
    // setting up the devices
    let device = Device::new_cuda(0)?; // using Cuda
    let dtype = DType::F32;
    // let device = Device::Cpu; // If using CPU
    // let dtype = DType::BF16; // unsupported for op matmul
    // let dtype = DType::F16;
    // below code brings the tokenizer to local machine, load & use 
    let start = std::time::Instant::now();
    // code below downloads tokenizer.json, the file that does the tokenizsation
    let tokenizer_filename = repo.get("tokenizer.json")?;
    let tokenizer_file = Tokenizer::from_file(tokenizer_filename.clone()).map_err(anyhow::Error::msg)?;
    let mut tokenizer = TokenOutputStream::new(tokenizer_file);
    // note the TokenOutputStream object is created using the .json file
    let prompt = "Where is the sun located in the Universe";
    // here we intialize the tokenizer
    let tokens = tokenizer
        .tokenizer()
        .encode(prompt, true)
        .map_err(E::msg)?
        .get_ids()
        .to_vec();
    // Following code takes the prompt and prints the tokenised data
    println!("The length of the tokens:{}", tokens.len());
    for &t in tokens.iter() {
        if let Some(t) = tokenizer.next_token(t)? {
            println!("{t}")
        }
        println!("Raw value is :{t}")
    }
    std::io::stdout().flush()?;
    // Next we will get the model related file
    // below code will bring the config.json first
    let config_filename = repo.get("config.json")?;
    // next get the model.safetensors.index.json
    let mdl_idx_json_file = candle_examples::hub_load_safetensors(&repo, "model.safetensors.index.json")?;
    // both files will be stored in ~/.cache/huggingface/hub/models--google--gemma-2-2b/ folder
    println!("retrieved the files in {:?}", start.elapsed());
    // building varbuilder, this is the Rust native object to store the model tensor
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&mdl_idx_json_file, dtype, &device)?};
    // above stores the tensors into memory
    let config: Config = serde_json::from_reader(std::fs::File::open(config_filename)?)?;
    // above line loads the config and makes it ready  for building the model
    let model = Model::new(false, &config, vb)?;
    // Final the mode is built and ready for Text Gen
    // turning flash_attn to false
    println!("loaded the tensors in memory {:?}", start.elapsed());
    // loading the model into the pipeline, again tokenizer has to be created
    // earlier tokenizers is consumed
    let tokenizer_file = Tokenizer::from_file(tokenizer_filename).map_err(anyhow::Error::msg)?;
    // the prompt is hard coded here
    let prompt = "Where is the sun located in the Universe";
    // TextGeneration pipeline
    let mut pipeline = TextGeneration::new(
        model, // Model
        tokenizer_file, // Tokenizer
        299792486, // Seed 
        Some(0.5), // top_p
        Some(0.7), // top_p
        1.1,
        64, 
        &device,// device to use, Cuda
    );
    pipeline.run(prompt, 100)?;   
    // pipeline is excuted
    println!("Completed inference in {:?}", start.elapsed());
    Ok(())
}
