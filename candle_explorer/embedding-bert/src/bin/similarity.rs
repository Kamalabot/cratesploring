#![allow(warnings)]
// The code is almost similar, when it comes to
// model loading and embedding process
use candle_transformers::models::bert::{BertModel, Config, HiddenAct, DTYPE};

use anyhow::{Error as E, Result};
use candle_core::Tensor;
use candle_nn::VarBuilder;
use clap::Parser;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::{PaddingParams, Tokenizer};
// We wont need this parser as we are hard coding the input
// however there are couple of other args in the code that still refer
// so keeping it
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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

    /// When set, compute embeddings for this prompt.
    // #[arg(long)]
    // prompt: Option<String>,

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
}

impl Args {
    // the following function still does the same process as shown in bert_embed code
    fn build_model_and_tokenizer(&self) -> Result<(BertModel, Tokenizer)> {
        let device = candle_examples::device(self.cpu)?;
        let default_model = "sentence-transformers/all-MiniLM-L6-v2".to_string();
        let default_revision = "refs/pr/21".to_string();
        let (model_id, revision) = match (self.model_id.to_owned(), self.revision.to_owned()) {
            (Some(model_id), Some(revision)) => (model_id, revision),
            (Some(model_id), None) => (model_id, "main".to_string()),
            (None, Some(revision)) => (default_model, revision),
            (None, None) => (default_model, default_revision),
        };

        let repo = Repo::with_revision(model_id, RepoType::Model, revision);
        let (config_filename, tokenizer_filename, weights_filename) = {
            let api = Api::new()?;
            let api = api.repo(repo);
            let config = api.get("config.json")?;
            let tokenizer = api.get("tokenizer.json")?;
            let weights = if self.use_pth {
                api.get("pytorch_model.bin")?
            } else {
                api.get("model.safetensors")?
            };
            (config, tokenizer, weights)
        };
        let config = std::fs::read_to_string(config_filename)?;
        let mut config: Config = serde_json::from_str(&config)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;

        let vb = if self.use_pth {
            VarBuilder::from_pth(&weights_filename, DTYPE, &device)?
        } else {
            unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DTYPE, &device)? }
        };
        if self.approximate_gelu {
            config.hidden_act = HiddenAct::GeluApproximate;
        }
        let model = BertModel::load(vb, &config)?;
        Ok((model, tokenizer))
    }
}
// Here the main function of similarity.rs starts...
fn main() -> Result<()> {
    use tracing_chrome::ChromeLayerBuilder;
    use tracing_subscriber::prelude::*;
    // args is dummy at this moment
    let args = Args::parse();
    let _guard = if args.tracing {
        println!("tracing...");
        let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        Some(guard)
    } else {
        None
    };
    // start the profiling
    let start = std::time::Instant::now();
    // getting the model and tokenizer in memory
    let (model, mut tokenizer) = args.build_model_and_tokenizer()?;
    let device = &model.device;
    // We have 8 sentences that is hard coded as array
    let sentences = [
        "The cat sits outside",
        "A man is playing guitar",
        "I love pasta",
        "The new movie is awesome",
        "The cat plays in the garden",
        "A woman watches TV",
        "The new movie is so great",
        "Do you like pizza?",
    ];
    let n_sentences = sentences.len(); // will store value of 8
                                       // following is the padding strategy followed by tokenizer as the
                                       // sentence lengths are different
    if let Some(pp) = tokenizer.get_padding_mut() {
        pp.strategy = tokenizers::PaddingStrategy::BatchLongest
    } else {
        let pp = PaddingParams {
            strategy: tokenizers::PaddingStrategy::BatchLongest,
            ..Default::default()
        };
        tokenizer.with_padding(Some(pp));
    }
    // encoding of the sentences into tokens
    let tokens = tokenizer
        .encode_batch(sentences.to_vec(), true)
        .map_err(E::msg)?;
    let token_ids = tokens
        .iter()
        .map(|tokens| {
            let tokens = tokens.get_ids().to_vec();
            Ok(Tensor::new(tokens.as_slice(), device)?)
        })
        .collect::<Result<Vec<_>>>()?;
    // attention mask is created for sending it into embedding model
    // its required
    let attention_mask = tokens
        .iter()
        .map(|tokens| {
            let tokens = tokens.get_attention_mask().to_vec();
            Ok(Tensor::new(tokens.as_slice(), device)?)
        })
        .collect::<Result<Vec<_>>>()?;
    // token_ids and attention_mask are stacked in next 2 lines
    let token_ids = Tensor::stack(&token_ids, 0)?;
    let attention_mask = Tensor::stack(&attention_mask, 0)?;
    let token_type_ids = token_ids.zeros_like()?;
    // starting the inference for 8 X 8 batch of sentences
    println!("running inference on batch {:?}", token_ids.shape());
    // embedding all the sentence in single forward pass
    let embeddings = model.forward(&token_ids, &token_type_ids, Some(&attention_mask))?;
    // print the embedding shape... Lets seee the demo again
    println!("generated embeddings {:?}", embeddings.shape());
    // as we see, the sentences are now vector of dim 384
    println!("Time elapsed in generating embedding {:?}", start.elapsed());

    // Apply some avg-pooling by taking the mean embedding value for all tokens (including padding)
    let (_n_sentence, n_tokens, _hidden_size) = embeddings.dims3()?;
    // starting to calculate the distances between embeddings of the sentences
    let embeddings = (embeddings.sum(1)? / (n_tokens as f64))?;
    // normalize_embeddings to avoid -ve or outliers numbers
    let embeddings = if args.normalize_embeddings {
        normalize_l2(&embeddings)?
    } else {
        embeddings
    };
    println!("pooled embeddings {:?}", embeddings.shape());
    // similarities are stored as vectors
    let mut similarities = vec![];
    // each sentence is enumerated
    for i in 0..n_sentences {
        // sentence embedding is assigned
        let e_i = embeddings.get(i)?;
        // the below loop takes care of the rest
        // of the sentences except the current
        for j in (i + 1)..n_sentences {
            let e_j = embeddings.get(j)?;
            // euclidean distance is calculated below
            let sum_ij = (&e_i * &e_j)?.sum_all()?.to_scalar::<f32>()?;
            let sum_i2 = (&e_i * &e_i)?.sum_all()?.to_scalar::<f32>()?;
            let sum_j2 = (&e_j * &e_j)?.sum_all()?.to_scalar::<f32>()?;
            let cosine_similarity = sum_ij / (sum_i2 * sum_j2).sqrt();
            // similarity between the sentences are pushed into above
            // similarities vector
            similarities.push((cosine_similarity, i, j))
        }
    }
    // before the similarities are printed, they are sorted
    similarities.sort_by(|u, v| v.0.total_cmp(&u.0));
    // similarity between current and the next sentence is printed
    for &(score, i, j) in similarities[..5].iter() {
        println!("score: {score:.2} '{}' '{}'", sentences[i], sentences[j])
    }
    // Finally the time elapsed is shared....
    println!(
        "Time elapsed in calculating similarity {:?}",
        start.elapsed()
    );
    Ok(())
    // thats the end of the code walkthrough...
}

pub fn normalize_l2(v: &Tensor) -> Result<Tensor> {
    Ok(v.broadcast_div(&v.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}
