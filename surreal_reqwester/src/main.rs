#![allow(warnings)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::format};

use reqwest::{get, Client};
use serde_json::{from_str, json, to_string};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let server = "http://127.0.0.1:3000/";
    let baseresp = client.get(server).send().await?.text().await?;
    println!("Base resp: {}", baseresp);
    let employee_data = json!({
        "name":"name1",
        "department":"dept1",
        "position":"top",
        "salary": 675567.8
    });
    // let post_res = client
    //     .post("http://127.0.0.1:3000/employees")
    //     .json(&employee_data)
    //     .send()
    //     .await?
    //     .text()
    //     .await?;
    // println!("Post resp: {:?}", post_res);
    let emp_resp = client
        // .get("http://127.0.0.1:3000/employee/⟨employee:2da66879-ef61-45f0-b3cb-b4e4526790e7⟩")
        .get("http://127.0.0.1:3000/allemps")
        .send()
        .await?
        // .text()
        .json()
        .await?;
    println!("The employees are: {:?}", emp_resp);
    Ok(())
}
