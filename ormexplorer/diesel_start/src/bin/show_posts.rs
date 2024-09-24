#![allow(unused_imports)]

use self::models::*;
use diesel::prelude::*;
use diesel_start::*;

fn main() {
    use self::schema::posts::dsl::*;
    let conn = &mut estab_conn();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(conn)
        .expect("Error Loading Posts");

    println!("Displaying {} results", results.len());
    for post in results {
        println!("{}", post.title);
        println!("{}", post.body);
    }
}
