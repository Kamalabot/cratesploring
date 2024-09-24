pub mod models;
pub mod schema;

use self::models::{NewPost, Post};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn estab_conn() -> PgConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect(".env file must contain DATABASE_URL");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Error in connection: {}", db_url))
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };
    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving post")
}
