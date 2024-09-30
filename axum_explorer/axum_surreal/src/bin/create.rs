#![allow(warnings)]
#![allow(unused_imports)]
use axum_surreal::Employee;
use clap::Parser;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::{engine::remote::ws::Ws, Surreal};

#[derive(Debug, Serialize, Deserialize)]
struct Rec {
    id: Thing,
}

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    /// Provide the name
    name: String,
    /// Provide the department
    department: String,
    /// Provide the position
    position: String,
    /// Provide the salary
    salary: f64,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let client = Surreal::new::<Ws>("localhost:8000").await.unwrap();
    client
        .signin(Root {
            username: "root",
            password: "root",
        })
        .await
        .unwrap();
    client.use_ns("test_ns").use_db("test_db").await.unwrap();

    let emp = Employee {
        id: None,
        name: cli.name,
        department: cli.department,
        position: cli.position,
        salary: cli.salary,
    };
    let thing: Option<Rec> = client
        .create("employee")
        .content(emp.clone())
        .await
        .unwrap();
    println!("Employee created. Check by running select.rs");
}
