use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Debug, Serialize)]
struct Name<'a> {
    first: &'a str,
    last: &'a str,
}

#[derive(Debug, Serialize)]
struct Person<'a> {
    title: &'a str,
    name: Name<'a>,
    marketing: bool,
}

#[derive(Debug, Serialize)]
struct Responsibility {
    marketing: bool,
}

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    // after connecting above, signin below
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;
    // use_ns as below
    db.use_ns("test").use_db("test").await?;
    // using create function, returns Record
    let created: Option<Record> = db
        .create("person")
        .content(Person {
            title: "Founder & CEO",
            name: Name {
                first: "Kamal",
                last: "M M",
            },
            marketing: true,
        })
        .await?;
    dbg!(created);
    let _c: Option<Record> = db
        .create("person")
        .content(Person {
            title: "employee",
            name: Name {
                first: "Nkloma",
                last: "",
            },
            marketing: false,
        })
        .await?;
    // update the below record, but doesn't work
    let updated: Option<Record> = db
        .update(("person", "Nkloma"))
        .merge(Responsibility { marketing: true })
        .await?;
    dbg!(updated);

    // look at the records
    let people: Vec<Record> = db.select("person").await?;

    dbg!(people);

    // println!("people are {:?}", people);

    let groups = db
        .query("SELECT marketing, count() FROM type::table($table) GROUP BY marketing")
        .bind(("table", "person"))
        .await?;
    dbg!(groups);

    Ok(())
}
