#![allow(unused_imports)]
#![allow(warnings)]


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
    pub id: Option<String>,
    pub name: String,
    pub department: String,
    pub position: String,
    pub salary: f64,
}
#[derive(Debug,Deserialize,Serialize)]
pub struct Record{
    pub id: Thing
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
    let up_emp = Employee{
            id: None,
            name:"name2".to_owned(),
            salary: 5576.8,
            department: "dept1".to_owned(),
            position: "pos1".to_owned(),
        };
    db.use_ns("test_ns").use_db("test_db").await.unwrap();
    let rec: Option<Record> = db.update(("employee", "4jey7dvf6tdi9dojmv1n"))
        .content(up_emp.clone())
        .await.unwrap();
    // println!("Extracted employees: {}", emp.len());
    println!(
        "Updated employees: {}",
        serde_json::to_string(&rec).unwrap()
    );
    println!("time elapsed: {:?}", start.elapsed());
}
