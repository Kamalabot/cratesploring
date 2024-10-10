use candle_core::{IndexOp, D};
use clap::{Parser, ValueEnum};

#[derive(Clone, Copy, Debug, ValueEnum)]
enum Which {
    SqueezeNet,
    EfficientN,
}

#[derive(Parser)]
struct Args {
    #[arg(long)]
    image: String,
    /// provide the full path of model.onnx
    #[arg(long)]
    model: Option<String>,

    /// The model to be used.
    #[arg(value_enum, long, default_value_t = Which::SqueezeNet)]
    which: Which,
}

pub fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // loading image from the given path
    let image = candle_examples::imagenet::load_image224(args.image)?;
    // depending on model chosen updating the image tensor
    let image = match args.which {
        Which::SqueezeNet => image,
        Which::EfficientN => image.permute((1, 2, 0))?,
    };

    println!("loaded image {image:?}");
    // try getting the model from which / model parameter
    let model = match args.model {
        // below is model.onnx file extension
        Some(model) => std::path::PathBuf::from(model),
        None => match args.which {
            // proceed to download the model
            Which::SqueezeNet => hf_hub::api::sync::Api::new()?
                .model("lmz/candle-onnx".into())
                .get("squeezenet1.1-7.onnx")?,
            Which::EfficientN => hf_hub::api::sync::Api::new()?
                .model("onnx/EfficientNet-Lite4".into())
                .get("efficientnet-lite4-11.onnx")?,
        },
    };
    // read the model into onnx
    let model = candle_onnx::read_file(model)?;
    // create graph instance
    let graph = model.graph.as_ref().unwrap();
    // mutable inputs hashmap
    let mut inputs = std::collections::HashMap::new();
    // creating the inputs with string and image tensor from above
    inputs.insert(graph.input[0].name.to_string(), image.unsqueeze(0)?);
    // running the simple_eval function with model & inputs
    let mut outputs = candle_onnx::simple_eval(&model, inputs)?;
    // extracting the output tensor, and discarding string
    let output = outputs.remove(&graph.output[0].name).unwrap();
    // get the probabilities
    let prs = match args.which {
        Which::SqueezeNet => candle_nn::ops::softmax(&output, D::Minus1)?,
        Which::EfficientN => output,
    };
    // to vectors
    let prs = prs.i(0)?.to_vec1::<f32>()?;

    // Sort the predictions and take the top 5
    let mut top: Vec<_> = prs.iter().enumerate().collect();
    top.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());
    let top = top.into_iter().take(5).collect::<Vec<_>>();

    // Print the top predictions
    for &(i, p) in &top {
        println!(
            "{:50}: {:.2}%",
            // here the imagenet is containing the classes
            candle_examples::imagenet::CLASSES[i],
            p * 100.0
        );
    }

    Ok(())
}
