#![allow(unused_imports)]
#![allow(warnings)]

use std::os::unix::thread;

use serde::{Deserialize, Serialize};

// use surrealdb::sql::Value;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    Surreal,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Employee {
    pub id: Thing,
    pub name: String,
    pub department: String,
    pub position: String,
    pub salary: f64,
}
#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await.unwrap();
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .unwrap();
    db.use_ns("test_ns").use_db("test_db").await.unwrap();
    let emp: Vec<Employee> = db.select("employee").await.unwrap();
    // println!("Extracted employees: {}", emp.len());
    println!(
        "Extracted employees: {}",
        serde_json::to_string(&emp).unwrap()
    );
    println!("time elapsed: {:?}", start.elapsed());
}
