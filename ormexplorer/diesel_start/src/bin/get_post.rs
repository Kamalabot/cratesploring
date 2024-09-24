#![allow(unused_imports)]
use self::models::Post;
use diesel::prelude::*;
use diesel_start::*; // this is reqd
use std::env::args;

fn main() {
    use self::schema::posts::dsl::posts;

    let post_id = args()
        .nth(1)
        .expect("Get post requires 1 id")
        .parse::<i32>()
        .expect("Invalid Id");

    let conn = &mut estab_conn();

    let post = posts
        .find(post_id)
        .select(Post::as_select())
        .first(conn)
        .optional();

    match post {
        Ok(Some(post)) => println!("Post with id: {} has title: {}", post.id, post.title),
        Ok(None) => println!("No Post found"),
        Err(_) => println!("Error occured..."),
    }
}
