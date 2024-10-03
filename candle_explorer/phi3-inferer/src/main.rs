// https://huggingface.co/microsoft/phi-1_5/tree/main : 2.5GB
// https://huggingface.co/microsoft/phi-2/tree/main : 5.5 GB
// https://huggingface.co/microsoft/Phi-3-mini-4k-instruct : 7.5 GB
// Above are the details of the Microsoft's Phi3 model
// The file loads only a single model, phi1.5 and its based on
// https://github.com/huggingface/candle/blob/main/candle-examples/examples/phi/main.rs
// Used mainly for educational purpose & the medium blog

// flags
#![allow(warnings)]
#![allow(unused_imports)]
// start of imports
use anyhow::{Error as E, Result};
use clap::{Parser, ValueEnum};
// Tokenizer, Model, and Config
use candle_examples::token_output_stream::TokenOutputStream;
use candle_transformers::models::mixformer::{Config, MixFormerSequentialForCausalLM as MixFormer};
// will be using the below Config & Model structs
use candle_transformers::models::phi::{Config as PhiConfig, Model as Phi};
// Following are common requirements for all models
use candle_core::{DType, Device, IndexOp, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::Tokenizer;

// Will be using below struct in pipeline
// following is the code for pipeline
struct TextGeneration {
    model: Phi,
    device: Device,
    tokenizer: TokenOutputStream,
    logits_processor: LogitsProcessor,
    repeat_penalty: f32,
    repeat_last_n: usize,
    verbose_prompt: bool,
}

impl TextGeneration {
    #[allow(clippy::too_many_arguments)]
    fn new(
        // model, device, tokenizer, and supporting vars are declared
        model: Phi,
        tokenizer: Tokenizer,
        seed: u64,
        temp: Option<f64>,
        top_p: Option<f64>,
        repeat_penalty: f32,
        repeat_last_n: usize,
        verbose_prompt: bool,
        device: &Device,
    ) -> Self {
        let logits_processor = LogitsProcessor::new(seed, temp, top_p);
        Self {
            model,
            tokenizer: TokenOutputStream::new(tokenizer),
            logits_processor,
            repeat_penalty,
            repeat_last_n,
            verbose_prompt,
            device: device.clone(),
        }
    }
    // run function is key for the text generation process
    fn run(&mut self, prompt: &str, sample_len: usize) -> Result<()> {
        // model tokenizer is loaded
        use std::io::Write;
        println!("starting the inference loop");
        let tokens = self
            .tokenizer
            .tokenizer()
            .encode(prompt, true)
            .map_err(E::msg)?;
        // if prompt is empty, then bail out
        if tokens.is_empty() {
            anyhow::bail!("Empty prompts are not supported in the phi model.")
        }
        if self.verbose_prompt {
            // check if verbose is true
            for (token, id) in tokens.get_tokens().iter().zip(tokens.get_ids().iter()) {
                let token = token.replace('▁', " ").replace("<0x0A>", "\n");
                println!("{id:7} -> '{token}'");
            }
            // print the tokenised data
        }
        // getting the tokens ready for forward pass
        let mut tokens = tokens.get_ids().to_vec();
        let mut generated_tokens = 0usize;
        let eos_token = match self.tokenizer.get_token("<|endoftext|>") {
            Some(token) => token,
            None => anyhow::bail!("cannot find the endoftext token"),
        };
        print!("{prompt}");
        std::io::stdout().flush()?;
        let start_gen = std::time::Instant::now();
        let mut pos = 0;
        // there the model generation starts, number of
        // words / tokens generated = sample_len
        for index in 0..sample_len {
            let context_size = if index > 0 { 1 } else { tokens.len() };
            let ctxt = &tokens[tokens.len().saturating_sub(context_size)..];
            let input = Tensor::new(ctxt, &self.device)?.unsqueeze(0)?;
            // forward pass of the model. Observe , only the tensor of input variable i
            // is sent through
            let logits = self.model.forward(&input)?;
            let logits = logits.squeeze(0)?.to_dtype(DType::F32)?;
            // the outputs are seen as logit tensors
            let logits = if self.repeat_penalty == 1. {
                logits
            } else {
                let start_at = tokens.len().saturating_sub(self.repeat_last_n);
                // if there repeated words, then repeat repeat_penalty is applied
                candle_transformers::utils::apply_repeat_penalty(
                    &logits,
                    self.repeat_penalty,
                    &tokens[start_at..],
                )?
            };

            let next_token = self.logits_processor.sample(&logits)?;
            // all the tokens are pushed inside tokeniser
            tokens.push(next_token);
            generated_tokens += 1;
            if next_token == eos_token {
                // the tokenizer decodes the generated
                if let Some(t) = self.tokenizer.decode_rest()? {
                    print!("{t}");
                    std::io::stdout().flush()?;
                }
                break;
            }
            if let Some(t) = self.tokenizer.next_token(next_token)? {
                // the token is printed
                print!("{t}");
                std::io::stdout().flush()?;
                // as you saw the generation is a good sign
            }
            pos += context_size;
        }
        let dt = start_gen.elapsed();
        println!(
            // completed generation...
            "\n{generated_tokens} tokens generated ({:.2} token/s)",
            generated_tokens as f64 / dt.as_secs_f64(),
        );
        Ok(())
    }
}
// Main function starts here.
// Note the LunarVim provides the details of the object types
fn main() -> Result<()> {
    let start = std::time::Instant::now();
    // creating the connection with Huggingface repo

    let api = Api::new()?;
    // setting the model and revision
    let model_id = "microsoft/phi-1".to_string();
    let revision = "refs/pr/8".to_string();
    // using HF connection, and creating repo instance
    let repo = api.repo(Repo::with_revision(model_id, RepoType::Model, revision));
    // download tokeniser file
    let tokenizer_filename = repo.get("tokenizer.json")?;
    // download the model tensors
    let filenames = vec![repo.get("model.safetensors")?];

    println!("retrieved the files in {:?}", start.elapsed());
    // loading the files into tokenizers
    let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;

    let start = std::time::Instant::now();
    // loading the config and model file
    let config = Config::v1();
    let dtype = DType::F32;
    // note the Device is CPU, which confines the model to RAM
    // Lets check inference
    // As you see more tokens were created...
    let device = Device::Cpu;
    // lets continue, downloading config file
    let config_filename = repo.get("config.json")?;
    // loading the config file
    let config = std::fs::read_to_string(config_filename)?;
    let config: PhiConfig = serde_json::from_str(&config)?;
    // building the model using VarBuilder and downloaded safetensors
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&filenames, dtype, &device)? };
    // instantating new model
    let model = Phi::new(&config, vb)?;
    // acknowledge model loading
    println!("loaded the model in {:?}", start.elapsed());
    // Hard coding the prompt...
    let prompt = "Where is the sun located in universe?";
    // once the prompt is printed, we can say inference started
    // model is processing the indiviual
    let mut pipeline = TextGeneration::new(
        model,
        tokenizer,
        2997922458,
        Some(0.5),
        Some(0.7),
        1.1,
        64,
        true,
        &device,
    );
    // the types of all the params are self explanotors
    // below pipeline. will generate the token.s.
    pipeline.run(&prompt, 100)?;
    Ok(())
}
