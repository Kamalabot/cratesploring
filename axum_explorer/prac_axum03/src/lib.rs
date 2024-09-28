#![allow(unused_imports)]
#![allow(warnings)]

pub mod axum_funcs;
mod models;
mod schema;

use core::panic;
use diesel::{prelude::*, select};
use dotenvy::dotenv;
use std::env;

use models::{NewPracTrac, PracTrac};
use schema::practrac;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePrac {
    pub sessionname: String,
    pub practice: i32,
    pub package: String,
    pub completed: bool,
}

// create the CRUD function starting with connection
// make the function sync to start, later convert to async

pub fn connect() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("check if .env file is present");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Issue in db connection"))
}

// create prac session
pub fn create_prac(sessionname: String, practice: i32, package: String, completed: bool) {
    let mut conn = connect();
    let prac = NewPracTrac {
        sessionname,
        practice,
        package,
        completed,
    };
    diesel::insert_into(practrac::table)
        .values(&prac)
        .returning(PracTrac::as_returning())
        .get_result(&mut conn)
        .expect("insert failed");
    println!("Session created, check it with show or showall")
}

// show all prac session
pub fn show_all() {
    use self::schema::practrac::dsl::*;
    let mut conn = connect();
    let allprac = practrac
        .filter(completed.eq(true))
        .limit(10)
        .select(PracTrac::as_select())
        .load(&mut conn)
        .expect("Issue getting session");
    println!("Displaying results of {}", allprac.len());

    println!("{}", serde_json::to_string(&allprac).unwrap())
}
// show one prac session
pub fn show_prac(pid: i32) {
    use self::schema::practrac::dsl::*;
    let mut conn = connect();
    let prac = practrac
        .find(pid)
        .select(PracTrac::as_select())
        .first(&mut conn)
        .optional();

    match prac {
        Ok(Some(sess)) => println!("{:?}", serde_json::to_string(&sess)),
        Ok(None) => println!("{} not found", pid),
        Err(_) => println!("Querying Error"),
    }
}
// update one prac session
pub fn uncomplete(pid: i32) {
    use self::schema::practrac::dsl::*;
    let mut conn = connect();
    let prac_updt = diesel::update(practrac.find(pid))
        .set(completed.eq(false))
        .returning(PracTrac::as_returning())
        .get_result(&mut conn)
        .unwrap();
    print!(
        "Updated session Id: {} to {}",
        prac_updt.id, prac_updt.completed
    );
}
// delete one prac session
pub fn remove_sess(pid: i32) {
    use self::schema::practrac::dsl::*;

    let mut conn = connect();
    let emp = diesel::delete(practrac.filter(id.eq(pid)))
        .execute(&mut conn)
        .expect("Delete aborted");
    println!("Delete complete, check with show or show all");
}
