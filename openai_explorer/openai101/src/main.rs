use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use dotenvy::dotenv;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client = Client::new();

    let msg_pl = [
        ChatCompletionRequestSystemMessageArgs::default()
            .content("You are a helpful assistant")
            .build()?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content("Who is leader in meteorite mining")
            .build()?
            .into(),
        ChatCompletionRequestAssistantMessageArgs::default()
            .content("The AstroForge is the leader in Meteorite mining")
            .build()?
            .into(),
        ChatCompletionRequestUserMessageArgs::default()
            .content("Who is the CEO of the company")
            .build()?
            .into(),
    ];

    let req = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u32)
        .model("gpt-4o-mini")
        .messages(msg_pl)
        .build()?;
    let resp = client.chat().create(req).await?;
    println!("Response is: ");

    for choice in resp.choices {
        println!(
            "{}: {} Role content: {:?}",
            choice.index, choice.message.role, choice.message.content
        );
    }
    Ok(())
}
