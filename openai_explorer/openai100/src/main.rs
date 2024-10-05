use async_openai::types::CreateCompletionRequestArgs;
use async_openai::Client;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = Client::new();
    let req = CreateCompletionRequestArgs::default()
        .model("gpt-3.5-turbo-instruct")
        .prompt("Provide me the OpenCV code for measuring dimensions of 3d object")
        .max_tokens(600u32)
        .build()
        .unwrap();

    let resp = client.completions().create(req).await.unwrap();

    println!("{:?}", resp.choices.first().unwrap().text)
}
