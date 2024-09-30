#![allow(warnings)]
#![allow(unused_imports)]

use kalosm::language::*;

#[derive(Parse, Clone, Debug)]
enum Class{
    Thing,
    Person,
    Animal
}

#[derive(Parse,Clone, Debug)]
struct Response {
    classified: Class
}

#[tokio::main]
async fn main(){
    let llm = Llama::new_chat().await.unwrap();
    let task = Task::builder("You classify the user message as about a person, animal or thing in json format response")
        .with_constraints(Response::new_parser())
        .build();
    let resp = task.run("There is phenomenal looking computer monitor there, come lets see", &llm).await.unwrap();
    println!("Prompt: There is phenomenal looking computer monitor there, come lets see");
    println!("{:?}", resp);
}
