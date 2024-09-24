use diesel_start::*;
use std::io::{stdin, Read};

fn main() {
    let conn = &mut estab_conn();
    let mut title = String::new();
    let mut body = String::new();

    println!("Add the title:");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("\nOK.. Lets work on {title}, when done press Ctrl+D");
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(conn, title, &body);
    println!("\n Saved draft {title} with id {}", post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
