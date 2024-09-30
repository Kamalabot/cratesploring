#[allow(warnings)]
#[allow(unused_imports)]
use dotenvy::dotenv;
use kalosm::language::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut llm = Gpt4::default();
    let prompt = "Structure the following into json. name is name1, phone is 578675678 and salary is 578675678.";
    println!("The prompt: {}", prompt);
    let mut stream = llm.stream_text(prompt).with_max_length(300).await.unwrap();
    stream.to_std_out().await.unwrap();
    Ok(())
}
