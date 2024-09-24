#![allow(unused_imports)]
#![allow(warnings)]

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct InMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body {
    pub model: String,
    pub messages: Vec<InMessage>,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: Function,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub paramters: Parameters,
    pub required: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Parameters {
    #[serde(rename = "type")]
    pub param_type: String,
    pub properties: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FuncBody {
    pub model: String,
    pub messages: Vec<InMessage>,
    pub tools: Vec<Tool>,
    pub tool_choice: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Payload {
    pub query: String,
}

pub fn get_func_body(payload: Payload) -> Value {
    let inmsg = InMessage {
        role: "user".to_owned(),
        content: payload.query,
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
        name: "get_current_weather".to_owned(),
        description: "Get the current weather in a given location".to_owned(),
        paramters: params,
        required: vec!["location".to_owned()],
    };
    let tool = Tool {
        tool_type: "function".to_owned(),
        function,
    };
    let funcbody = FuncBody {
        model: "gpt-4o-mini".to_owned(),
        messages: vec![inmsg],
        tools: vec![tool],
        tool_choice: "auto".to_owned(),
    };
    serde_json::to_value(&funcbody).unwrap().into()
}
