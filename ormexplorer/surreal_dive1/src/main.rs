use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize)]
struct Logik {
    name: String,
    #[serde(rename = "type")]
    type_str: String,
    make: String,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let sdb = Surreal::new::<Ws>("localhost:8000".to_owned()).await?;
    //signin
    sdb.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    sdb.use_ns("test").use_db("test").await?;
    //insert data
    // let _user: Option<Record> = sdb
    //     .create("logic")
    //     .content(serde_json::json!({
    //         "name": "greater",
    //         "type": "comparison",
    //         "make": "boolean"
    //     }))
    //     .await?;
    // let get_logic: Vec<Record> = sdb.select("logic").await?;
    // dbg!(&get_logic);
    // println!("Extracted logic: {:?}", get_logic[0]);

    // let again: Vec<Logik> = sdb.select("logic").await?;
    // println!("Total number of items are: {}", again.len());
    // println!("again extracted: {:?}", again[0]);

    // let mut one: Option<Logik> = sdb.select(("logic", "logic:1f2oq49fkl8px550mcq7")).await?;
    let mut one: Option<Logik> = sdb.select(("logic", "1f2oq49fkl8px550mcq7")).await?;
    println!("Getting one: {:?}", one.take());

    let mut edit_logic: Option<Logik> = sdb
        .update(("logic", "1f2oq49fkl8px550mcq7"))
        .content(serde_json::json!({
            "name": "greater",
            "type": "magnanimous",
            "make": "binmanial"
        }))
        .await?;
    println!("look at updated user: {:?}", edit_logic.take());

    let _dlo: Option<Record> = sdb.delete(("logic", "1f2oq49fkl8px550mcq7")).await?;
    Ok(())
}
