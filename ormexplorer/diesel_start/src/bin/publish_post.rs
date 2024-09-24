#![allow(unused_imports)]

use self::models::Post;
use diesel::prelude::*;
use diesel_start::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish post requires a post id")
        .parse::<i32>()
        .expect("Invalid Id");

    let conn = &mut estab_conn();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .returning(Post::as_returning())
        .get_result(conn)
        .unwrap();

    println!("Published post {}", post.title);
}
