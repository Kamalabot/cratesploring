use candle_core::{Device, Tensor, DType};
use anyhow::Result;
use hf_hub::api::sync::Api;
use candle_nn::{Linear, Module};

fn main() -> Result<()> {
    let dev = &Device::Cpu;
    let api = Api::new().unwrap();
    let repo = api.model("bert-base-uncased".to_string());
    let weight = repo.get("model.safetensors").unwrap();
    let loaded = candle_core::safetensors::load(weight, dev)?;

    for (k, v) in loaded.iter() {
        println!("The key: {:?} and value {:?}", k, v);
    }
    
    let layers = loaded.values();

    let elems = layers.len();

    println!("The number of layers: {}", elems);

    let wt1 = loaded.get("bert.encoder.layer.0.attention.self.query.weight").unwrap();
    let bi1 = loaded.get("bert.encoder.layer.0.attention.self.query.bias").unwrap();

    let mklayer = Linear::new(wt1.clone(), Some(bi1.clone()));

    let input_ids = Tensor::zeros((3, 768), DType::F32, dev).unwrap();

    let output = mklayer.forward(&input_ids)?;

    println!("The output is {:?}", output);
    Ok(())
}
