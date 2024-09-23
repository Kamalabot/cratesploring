#![allow(unused_imports)]
#![allow(warnings)]

use std::iter::FromIterator;
use std::{env, error::Error};
use tokio;

use reqwest::{
    header::{self, HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub content: String,
}

pub async fn get_response(query: String, tokens: u32) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let body = &get_body(query.clone(), tokens);
    // println!("{:?}", body);
    let funcbody = &get_func_body(query.clone());
    println!("{:?}", funcbody);
    // let response: ApiResponse = client
    //     .post(url)
    //     .headers(get_header())
    //     .json(body)
    //     .send()
    //     .await?
    //     .json()
    //     .await?;
    // println!("Response: {:?}", response);
    // Ok(response)
    Ok(())
}

pub fn get_body(query: String, tokens: u32) -> serde_json::Value {
    // json!(
    //     {
    //         "model":"gpt-4o-mini",
    //         "messages":[
    //             // {"role": "system",
    //             // "content": get_system_message()
    //             // },
    //         {
    //             "role":"user",
    //             "content": query,
    //         }
    //         ],
    //         "max_tokens": tokens,
    //     }
    // )
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

#[derive(Debug, Deserialize, Serialize)]
pub struct InMessage {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body {
    model: String,
    messages: Vec<InMessage>,
    max_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tool {
    tool_type: String,
    function: Function,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Function {
    name: String,
    description: String,
    paramters: Parameters,
    required: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    param_type: String,
    properties: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FuncBody {
    model: String,
    message: Vec<InMessage>,
    tools: Vec<Tool>,
    tool_choice: String,
}

pub fn get_func_body(query: String) -> serde_json::Value {
    let inmsg = InMessage {
        role: "user".to_owned(),
        content: query,
    };

    let props = json!({
        "location": {
            "type": "string",
            "description": "The city and state, e.g. San Francisco, CA"
            },
        "unit": {
            "type": "string",
            "enum": ["celsius", "fahrenheit"]
            }
    });
    let params = Parameters {
        param_type: "object".to_owned(),
        properties: props,
    };

    let function = Function {
        name: "get get_current_weather".to_owned(),
        description: "Get the current weather in a given location".to_owned(),
        paramters: params,
        required: vec!["location".to_owned()],
    };
    let tool = Tool {
        tool_type: "required".to_owned(),
        function,
    };
    let funcbody = FuncBody {
        model: "gpt-4o-mini".to_owned(),
        message: vec![inmsg],
        tools: vec![tool],
        tool_choice: "auto".to_owned(),
    };
    serde_json::to_value(&funcbody).unwrap()
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
    let completion = get_response("Are there robots on moon?".to_owned(), 100).await?;
    Ok(())
}
