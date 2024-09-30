#![allow(warnings)]
#![allow(unused_imports)]

use std::io::Write;
use kalosm::{*, language::*};

#[tokio::main]
async fn main() {
    let mut llm = Llama::new().await.unwrap();
    let prompt = "The following is 100 words essay on rust ";
    let mut stream = llm.stream_text(prompt).with_max_length(1000).await.unwrap();
    stream.to_std_out().await.unwrap();
}
