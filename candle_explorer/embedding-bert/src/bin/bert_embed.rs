// This is the part1
// Import flags
#![allow(warnings)]

// We see the BertModel & its rust native config is imported
use candle_transformers::models::bert::{BertModel, Config, HiddenAct, DTYPE};

use anyhow::{Error as E, Ok, Result};
// Candle_core and candle_nn crates are used for building the model
use candle_core::Tensor;
use candle_nn::VarBuilder;
// clap is the command line parser, we are deriving the Parser from it
use clap::Parser;
// hf_hub api provides the interface to download the models
use hf_hub::{api::sync::Api, Repo, RepoType};
// tokenizer to split the sentence & encode it to numbers
use tokenizers::{PaddingParams, Tokenizer};

// We start by coding the CLI parser
// The help that we saw in the CLI is automatically
// generated using the below struct that derives CLAP's Parser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// creating the Args struct, the base for the parser
struct Args {
    /// Run on CPU rather than on GPU.
    #[arg(long)]
    cpu: bool,

    /// Enable tracing (generates a trace-timestamp.json file).
    #[arg(long)]
    tracing: bool,

    /// The model to use, check out available models: https://huggingface.co/models?library=sentence-transformers&sort=trending
    #[arg(long)]
    model_id: Option<String>,

    #[arg(long)]
    revision: Option<String>,
    // let me show you the extended help in cli now
    /// When set, compute embeddings for this prompt.
    #[arg(long)]
    prompt: String,

    /// Use the pytorch weights rather than the safetensors ones
    #[arg(long)]
    use_pth: bool,

    /// The number of times to run the prompt.
    #[arg(long, default_value = "1")]
    n: usize,

    /// L2 normalization for embeddings.
    #[arg(long, default_value = "true")]
    normalize_embeddings: bool,

    /// Use tanh based approximation for Gelu instead of erf implementation.
    #[arg(long, default_value = "false")]
    approximate_gelu: bool,
    // as you saw in the cli help, all the above args
    // are available as options to the application.. Neat rite??
}

impl Args {
    // following function pulls, loads and processes the prompt...
    // In the demo.. the models were already downloaded. When you will
    // execute, the model will download
    fn build_model_and_tokenizer(&self) -> Result<(BertModel, Tokenizer)> {
        // setting the device
        let device = candle_examples::device(self.cpu)?;
        // we are using the default model as all-MiniLM-L6-v2
        // we can also provide the another model in the cli...
        // we will see the demo now..
        // we saw how the sentence-transformers/multi-qa-MiniLM-L6-cos-v1 was downloaded
        // and used for embedding
        let default_model = "sentence-transformers/all-MiniLM-L6-v2".to_string();
        let default_revision = "refs/pr/21".to_string();
        // following lines of code parse the recieved CLI args in rust native way
        let (model_id, revision) = match (self.model_id.to_owned(), self.revision.to_owned()) {
            (Some(model_id), Some(revision)) => (model_id, revision),
            (Some(model_id), None) => (model_id, "main".to_string()),
            (None, Some(revision)) => (default_model, revision),
            (None, None) => (default_model, default_revision),
        };
        // Below the repo instances are creaated on the model_id provided
        let repo = Repo::with_revision(model_id, RepoType::Model, revision);
        // following lines download the files
        let (config_filename, tokenizer_filename, weights_filename) = {
            let api = Api::new()?;
            let api = api.repo(repo);
            // download config.json
            let config = api.get("config.json")?;
            // get tokenizer.json
            let tokenizer = api.get("tokenizer.json")?;
            // get the actual model bin / safetensors file
            let weights = if self.use_pth {
                api.get("pytorch_model.bin")?
            } else {
                api.get("model.safetensors")?
            };
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
        let vb = if self.use_pth {
            VarBuilder::from_pth(&weights_filename, DTYPE, &device)?
        } else {
            unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)? }
        };
        // the activation method in the embedding is modified below
        if self.approximate_gelu {
            config.hidden_act = HiddenAct::GeluApproximate;
        }
        let model = BertModel::load(vb, &config)?;
        Ok((model, tokenizer))
        // model and tokenizers are returned
    }
}
// The main function starts here
fn main() -> Result<()> {
    use tracing_chrome::ChromeLayerBuilder;
    use tracing_subscriber::prelude::*;
    // above imports are for tracing and tracking. Will discuss it in another vid
    // Below is the args variable that parses the CLI args
    // All the args that were highlighted, will be inside below variable
    let args = Args::parse();
    // checks if tracing is enabled
    let _guard = if args.tracing {
        println!("tracing...");
        let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        Some(guard)
    } else {
        None
    };
    // timing the execution using start::Instant
    let start = std::time::Instant::now();
    // calling the build_model_and_tokenizer function and getting the model & tokenizer
    let (model, mut tokenizer) = args.build_model_and_tokenizer()?;
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
        .encode(args.prompt, true)
        .map_err(E::msg)?
        .get_ids()
        .to_vec();
    // token ids are made ready for embedding
    let token_ids = Tensor::new(&tokens[..], device)?.unsqueeze(0)?;
    let token_type_ids = token_ids.zeros_like()?;
    println!("Loaded and encoded {:?}", start.elapsed());
    // below the embedding process is done using the forward function
    for idx in 0..args.n {
        let start = std::time::Instant::now();
        let ys = model.forward(&token_ids, &token_type_ids, None)?;
        if idx == 0 {
            println!("{ys}");
        }
        println!("Took {:?}", start.elapsed());
    }
    Ok(())
    // The code walkthrough of part 1 is done...
}

pub fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}
