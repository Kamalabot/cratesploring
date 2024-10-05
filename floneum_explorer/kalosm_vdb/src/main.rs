#![allow(warnings)]
use kalosm_language::prelude::*;
use kalosm_language::rbert::*;
use kalosm_language_model::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("Creating default bert model");
    let bert = Bert::new_for_search().await.unwrap();
    let sentences = [
        "kalosm can be built for local AI applications",
        "With private LLMs data never leaves your machine",
        "The Quick brown fox jumps over the lazy dog",
    ];
    let embeddings = bert.embed_batch(sentences).await.unwrap();

    let db = VectorDB::new().unwrap();

    let embeddings = db.add_embeddings(embeddings).unwrap();

    let embedding_id_to_sentence: HashMap<EmbeddingId, &str> =
        HashMap::from_iter(embeddings.into_iter().zip(sentences));

    // now lets query??
    let query = "What is kalosm";

    let query_embed = bert.embed_query(query).await.unwrap();

    let closest = db.get_closest(query_embed, 1).unwrap();

    if let [closest] = closest.as_slice() {
        let dist = closest.distance;
        let vecid = closest.value;
        let text = embedding_id_to_sentence.get(&closest.value).unwrap();
        let embed = db.get_embedding(closest.value).unwrap().to_vec();
        println!("Distance:{dist}");
        println!("Embeddings: {:?}", embed);
        println!("Closest: {text}");
    }
}
