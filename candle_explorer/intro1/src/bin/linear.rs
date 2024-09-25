#[allow(dead_code)]

use candle_core::{Tensor, Device};
use anyhow::Result; // this result is different

struct Linear {
    weight: Tensor,
    bias: Tensor
}

impl Linear {
    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let x = x.matmul(&self.weight)?;
        let y = x.broadcast_add(&self.bias)?;
        Ok(y)
        // Ok(x.broadcast_add(&self.bias)?)
    }
}

struct Model {
    first: Linear,
    second: Linear,
}

impl Model {
    fn forward(&self, image: &Tensor) -> Result<Tensor> {
        let x = self.first.forward(image)?;
        let x = x.relu()?;
        Ok(self.second.forward(&x)?)
    }
}

fn main() -> Result<()> {
    let device = Device::cuda_if_available(0)?;
    // creating a Linear model
    let weight = Tensor::randn(0f32, 1.0, (784, 100), &device)?;
    let bias = Tensor::randn(0f32, 1.0, (100, ), &device)?;
    let first = Linear{weight, bias};

    // creating a Linear model
    let weight = Tensor::randn(0f32, 1.0, (100, 10), &device)?;
    let bias = Tensor::randn(0f32, 1.0, (10, ), &device)?;
    let second = Linear{weight, bias};
    
    let linmodel = Model { first, second };

    // take a dummy image.. 
    let dm_img = Tensor::randn(0f32, 1.0, (1, 784), &device)?;

    let digit = linmodel.forward(&dm_img)?;
    println!("the output is: {}", digit);
    Ok(())
}
