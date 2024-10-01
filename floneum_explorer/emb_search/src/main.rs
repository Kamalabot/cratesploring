#![allow(warnings)]
#![allow(unused_imports)]

use dotenvy::dotenv;
use kalosm::language::*;
use kalosm::surrealdb::{engine::local::RocksDb, Surreal};
use std::error::Error;
use std::path::PathBuf;

async fn extract_docs() -> Result<String, Box<dyn Error>> {
    // Read an RSS stream
    let nyt =
        RssFeed::new(Url::parse("https://rss.nytimes.com/services/xml/rss/nyt/US.xml").unwrap());
    // Read a local folder of documents
    let mut documents = DocumentFolder::try_from(PathBuf::from("./documents")).unwrap();
    // Read a website (either from the raw HTML or inside of a headless browser)
    let page = Page::new(
        Url::parse("https://www.nytimes.com/live/2023/09/21/world/zelensky-russia-ukraine-news")
            .unwrap(),
        BrowserMode::Static,
    )
    .unwrap();
    let document = page.article().await.unwrap();
    println!("Title: {}", document.title());
    println!("Body: {}", document.body());

    // Read pages from a search engine (You must have the SERPER_API_KEY environment variable set to run this example)
    let query = "What is the capital of France?";
    let api_key = std::env::var("SERPER_API_KEY").unwrap();
    let search_query = SearchQuery::new(query, &api_key, 5);
    let documents = search_query.into_documents().await.unwrap();
    let mut text = String::new();
    for document in documents {
        for word in document.body().split(' ').take(300) {
            text.push_str(word);
            text.push(' ');
        }
        text.push('\n');
    }
    println!("{}", text);
    Ok(text)
}

#[tokio::main]
async fn main() {
    // Create database connection
    let db = Surreal::new::<RocksDb>(std::env::temp_dir().join("temp.db"))
        .await
        .unwrap();

    // Select a specific namespace / database
    db.use_ns("search").use_db("documents").await.unwrap();

    // Create a table in the surreal database to store the embeddings
    let document_table = db
        .document_table_builder("documents")
        .build::<Document>()
        .await
        .unwrap();

    // Add documents to the database
    document_table
        .add_context(DocumentFolder::new("./documents").unwrap())
        .await
        .unwrap();

    loop {
        // Get the user's question
        let user_question = prompt_input("Query: ").unwrap();

        let nearest_5 = document_table
            .select_nearest(user_question, 5)
            .await
            .unwrap();
        println!("{:?}", nearest_5);
    }
}

async fn rag_process() -> anyhow::Result<()> {
    let exists = std::path::Path::new("./db").exists();

    // Create database connection
    let db = Surreal::new::<RocksDb>("./db/temp.db").await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Create a table in the surreal database to store the embeddings
    let document_table = db
        .document_table_builder("documents")
        .at("./db/embeddings.db")
        .build::<Document>()
        .await?;

    // If the database is new, add documents to it
    if !exists {
        std::fs::create_dir_all("documents")?;
        let context = [
            "https://floneum.com/kalosm/docs",
            "https://floneum.com/kalosm/docs/guides/retrieval_augmented_generation",
        ]
        .iter()
        .map(|url| Url::parse(url).unwrap());

        document_table.add_context(context).await?;
    }

    // Create a llama chat model
    let model = Llama::new_chat().await?;
    let mut chat = Chat::builder(model).with_system_prompt("The assistant help answer questions based on the context given by the user. The model knows that the information the user gives it is always true.").build();

    loop {
        // Ask the user for a question
        let user_question = prompt_input("\n> ")?;

        // Search for relevant context in the document engine
        let context = document_table
            .select_nearest(&user_question, 1)
            .await?
            .into_iter()
            .map(|document| {
                format!(
                    "Title: {}\nBody: {}\n",
                    document.record.title(),
                    document.record.body()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        // Format a prompt with the question and context
        let prompt = format!("{context}\n{user_question}");

        // Display the prompt to the user for debugging purposes
        println!("{}", prompt);

        // And finally, respond to the user
        let mut output_stream = chat.add_message(prompt);
        print!("Bot: ");
        output_stream.to_std_out().await?;
    }
}
