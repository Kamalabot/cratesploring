#![allow(warnings)]
#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use std::error::Error;
use surrealdb::engine::local::RocksDb;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Deserialize, Serialize)]
struct Rec {
    id: Thing,
}

#[derive(Debug, Deserialize, Serialize)]
struct Text {
    text: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let endpoint = "memory".to_owned();
    // In memory is not working
    // let db = any::connect(endpoint).await?;
    let db = Surreal::new::<RocksDb>(".temp.db").await?;
    // it has created file and loaded data into it.
    // can i do surreal sql connect?
    // you can after starting the surrealDb on it
    db.use_ns("test_ns").use_db("test_db").await?;
    let _rec: Option<Rec> = db
        .create("inmem".to_owned())
        .content(Text {
            text: "test text".to_string(),
        })
        .await?;
    let data: Vec<Text> = db.select("inmem").await?;
    println!("Number of rec: {:?}", data.len());
    println!("Records are: {:?}", data);
    Ok(())
}
