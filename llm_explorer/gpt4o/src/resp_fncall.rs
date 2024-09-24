use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct FnCallResponse {
    pub id: String,
    pub object: String,
    pub created: usize,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub completion_tokens_details: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
    pub message: FnMessage,
    pub finish_reason: String,
    pub index: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RespTool {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: RespFunction,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RespFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FnMessage {
    pub role: String,
    pub tool_calls: Vec<RespTool>,
}
