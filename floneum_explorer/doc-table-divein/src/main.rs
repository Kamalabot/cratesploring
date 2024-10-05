#![allow(warnings)]

use std::any::Any;

use kalosm::language::*;
use kalosm::surrealdb::{engine::local::RocksDb, Surreal};
use kalosm_language::prelude::*;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let exists = std::path::Path::new("./.temp.db").exists();

    let db = Surreal::new::<RocksDb>("./.temp.db").await.unwrap();

    db.use_ns("rag").use_db("rag").await.unwrap();
    println!("Database is connected...");

    let chunker = SemanticChunker::new();
    // same rocksDB database is used to store the
    // document table inside the database...
    let doc_table = db
        .document_table_builder("documents")
        .with_chunker(chunker)
        .at("./.embeddings.db")
        .build::<Document>()
        .await
        .unwrap();
    println!("Document table is built inside the db");

    if !exists {
        std::fs::create_dir_all("flnm_docs").unwrap();
        let context = [
            "https://floneum.com/kalosm/docs",
            "https://floneum.com//kalosm/docs/guides/retrieval_augmented_generation",
        ]
        .iter()
        .map(|url| Url::parse(url).unwrap());
        println!("Retrieved the text from urls");
        // here the context is added into the table
        let start = std::time::Instant::now();
        println!("The number of docs being added: {:?}", &context.len());
        println!("Enumerating the context");

        doc_table.add_context(context).await.unwrap();
        println!(
            "Time elapsed in adding context to embedding db {:?}",
            start.elapsed()
        );
        println!(
            "Vector DB typeID: {:?}",
            &doc_table.table().vector_db().type_id()
        );
    }
    loop {
        // below the user query starts
        let user_question = prompt_input("QUery: ").unwrap();
        let input = user_question.trim();
        if input == "all" {
            let get_all: Vec<Document> = doc_table.select_all().await.unwrap();
            println!("There are {} docs", get_all.len());
            for (idx, docall) in get_all.iter().enumerate() {
                println!("Doc: {idx}");
                println!("Doc body: {}", docall.body());
            }
        }
        let nearest2 = doc_table.select_nearest(user_question, 2).await.unwrap();
        for (idx, doc) in nearest2.iter().enumerate() {
            println!("Result {idx} details: ");
            println!("*****");
            println!("embedding id: {:?}", doc.id);
            println!("embedding id: {}", doc.record_id.to_string());
            println!(
                "language of document: {}",
                doc.record.language().unwrap().to_string()
            );
            println!("How far from query: {}", doc.distance);
            println!("document body: {}", doc.text());
        }
        println!("Use Ctrl+D to quit...");
    }
}
