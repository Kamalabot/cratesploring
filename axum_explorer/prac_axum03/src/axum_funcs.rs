use crate::models::{NewPracTrac, PracTrac};
use axum::{
    extract::{Json, Path},
    response::Html,
};
use core::panic;
use std::collections::HashMap;

use crate::schema::practrac;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePrac {
    pub sessionname: String,
    pub practice: i32,
    pub package: String,
    pub completed: bool,
}

pub async fn async_connect() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("missing .env file");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Connect failed"))
}

pub async fn path_finder(Path(id): Path<i32>) -> Html<String> {
    Html(format!("<h1>The Id is {}</h2>", id))
}

// create prac session
pub async fn async_create_prac(Json(payload): Json<CreatePrac>) -> Html<String> {
    let mut conn = async_connect().await;
    let prac = NewPracTrac {
        sessionname: payload.sessionname,
        practice: payload.practice,
        package: payload.package,
        completed: payload.completed,
    };
    diesel::insert_into(practrac::table)
        .values(&prac)
        .returning(PracTrac::as_returning())
        .get_result(&mut conn)
        .expect("insert failed");
    Html(format!("Session created, check it with show or showall"))
}

// show all prac session
pub async fn async_show_all() -> Json<Value> {
    use crate::schema::practrac::dsl::*;
    let mut conn = async_connect().await;
    let allprac = practrac
        .filter(completed.eq(true))
        .limit(10)
        .select(PracTrac::as_select())
        .load(&mut conn)
        .expect("Issue getting session");
    println!("Displaying results of {}", allprac.len());

    Json(serde_json::to_value(&allprac).unwrap())
}
// show one prac session
pub async fn async_show_prac(Path(pid): Path<i32>) -> Json<Value> {
    use crate::schema::practrac::dsl::*;
    let mut conn = async_connect().await;
    let prac = practrac
        .find(pid)
        .select(PracTrac::as_select())
        .first(&mut conn)
        .optional();

    match prac {
        Ok(Some(sess)) => Json(serde_json::to_value(&sess).unwrap()),
        Ok(None) => Json(json!("{\"pid\": pid, \"status\": \"Not Found\"}")),
        Err(_) => Json(json!("{\"pid\": pid, \"status\": \"Query Error\"}")),
    }
}
// update one prac session
pub async fn async_uncomplete(Path(pid): Path<i32>) -> Json<Value> {
    use crate::schema::practrac::dsl::*;
    let mut conn = async_connect().await;
    let prac_updt = diesel::update(practrac.find(pid))
        .set(completed.eq(false))
        .returning(PracTrac::as_returning())
        .get_result(&mut conn)
        .unwrap();
    Json(json!(
        "{\"pid\": pid, \"status\": \"Updated\", \"Completed\":prac_updt.completed}"
    ))
}

// delete one prac session
pub async fn async_remove_sess(Path(pid): Path<i32>) -> Json<Value> {
    use crate::schema::practrac::dsl::*;

    let mut conn = async_connect().await;
    let emp = diesel::delete(practrac.filter(id.eq(pid)))
        .execute(&mut conn)
        .expect("Delete aborted");

    Json(json!("{\"pid\": pid, \"status\": \"Deleted\"}"))
}
