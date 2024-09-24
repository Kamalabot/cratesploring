#![allow(unused_imports)]
#![allow(warnings)]
mod fncall;
mod resp_completion;
mod resp_fncall;
use crate::fncall::*;
use crate::resp_completion::*;
use crate::resp_fncall::*;

use std::iter::FromIterator;
use std::{env, error::Error};
use tokio;

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub async fn get_response(query: String, tokens: u32) -> Result<FnCallResponse, Box<dyn Error>> {
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    // let body = &get_body(query.clone(), tokens);
    // println!("{:?}", body);
    let payload = Payload { query };
    let funcbody = get_func_body(payload);
    // println!("{:?}", funcbody);
    let response: Value = client
        .post(url)
        .headers(get_header())
        .json(&funcbody)
        .send()
        .await?
        .json()
        .await?;
    // this is parsing from response to FnCallResponse
    let fncall: FnCallResponse = serde_json::from_value(response)?;
    // this is parsing from FnCallResponse to String
    let fncall_string = serde_json::to_string(&fncall)?;
    println!("String_response: {}", fncall_string);
    // this is extracting the function name from the FnCallResponse
    let function_name = &fncall.choices[0].message.tool_calls[0].function.name;
    // println!("Function to call: {}", function_name);
    println!("Function name: {}", function_name);
    Ok(fncall)
    // Ok(())
}

pub fn get_body(query: String, tokens: u32) -> serde_json::Value {
    let message = InMessage {
        role: "user".to_owned(),
        content: query.to_owned(),
    };
    let body = Body {
        model: "gpt-4o-mini".to_owned(),
        messages: vec![message],
        max_tokens: tokens,
    };
    let body_string = serde_json::to_value(&body).unwrap();
    body_string
}

pub fn get_header() -> HeaderMap<HeaderValue> {
    header::HeaderMap::from_iter(vec![
        (header::CONTENT_TYPE, "application/json".parse().unwrap()),
        (
            header::AUTHORIZATION,
            format!("Bearer {}", get_api_key()).parse().unwrap(),
        ),
    ])
}

pub fn get_api_key() -> String {
    env::var("OPEN_AI_API_KEY").expect("OPEN_AI_API_KEY not set")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let completion = get_response("What is the temperature Tamilnadu".to_owned(), 100).await?;
    Ok(())
}
