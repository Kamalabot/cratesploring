#[allow(dead_code)]
#[allow(unused_imports)]

use candle_core::{DType, Device, Tensor};
use anyhow::Result;

struct Model {
    first: Tensor,
    second: Tensor
} // this part is like init() part of Model Class

impl Model {
    // this is like the forward method
    fn forward(&self, image: &Tensor) -> Result<Tensor> {
        let x = image.matmul(&self.first)?;
        let x = x.relu()?;
        Ok(x.matmul(&self.second)?) // the Result data after forward pass is returned
    }
}
fn main() -> Result<()> {

    let data = [1u32, 2, 3, 5]; // array is created
    // can we create vector and send to tensors?
    // let device = Device::new_cuda(0)?;
    let device = Device::Cpu; // The device is selected here
    let tensor = Tensor::new(&data, &device).unwrap();
    println!("The tensor test: {}", tensor);

    // Mnist Model tutorial
    let fst = Tensor::randn(0f32, 1.0, (784, 100), &device)?;
    let scd = Tensor::randn(0f32, 1.0, (100, 10), &device)?;
    let model = Model { first:fst, second:scd };

    let dm_img = Tensor::randn(0f32, 1.0, (1, 784), &device)?;

    let digit = model.forward(&dm_img)?;

    println!("Digit: {}", digit);

    Ok(())

}
