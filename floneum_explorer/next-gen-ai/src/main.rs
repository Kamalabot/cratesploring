#![allow(warnings)]
#![allow(unused_imports)]

use std::io::Write;
use std::io::stdin;
use kalosm::{*, language::*};

#[tokio::main]
async fn main() {
    let mut llm = Llama::new().await.unwrap();
    let mut prompt = String::new(); 
    loop {

        println!("Type exit or else");
        println!("Ask your question: ");
        stdin().read_line(&mut prompt).unwrap();
        if prompt.trim() == "exit"{
            break
        }
        println!("The LLM reply: ");
        let mut stream = llm.stream_text(&prompt).with_max_length(250).await.unwrap();
        stream.to_std_out().await.unwrap();
    }
}
