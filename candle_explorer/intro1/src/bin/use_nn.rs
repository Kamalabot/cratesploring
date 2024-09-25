use anyhow::Ok;
#[allow(dead_code)]

use candle_core::{Device, Tensor};
use candle_nn::{Linear, Module};
use anyhow::Result;

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
    let device = Device::Cpu;

    let weight = Tensor::randn(0f32, 1.0, (100, 784), &device)?;
    let bias = Tensor::randn(0f32, 1.0, (100, ), &device)?; 

    let first = Linear::new(weight, Some(bias));
    
    let weight = Tensor::randn(0f32, 1.0, (10, 100), &device)?;
    let bias = Tensor::randn(0f32, 1.0, (10, ), &device)?; 

    let second = Linear::new(weight, Some(bias));

    let mdl = Model { first, second };

    let img = Tensor::randn(0f32, 1.0, (1, 784), &device)?;

    let dgt = mdl.forward(&img)?;

    println!("the digit is: {dgt:?}");

    Ok(())

}
