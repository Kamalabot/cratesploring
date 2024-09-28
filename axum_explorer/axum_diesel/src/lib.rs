#![allow(unused_imports)]
#![allow(warnings)]

mod models;
mod schema;

use axum::extract::{Json, Path, Query};
use diesel::{prelude::*, select};
use dotenvy::dotenv;

use models::{AxumEmployee, NewAxumEmployee};
use schema::axumemployee;

use std::env;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateEmployee {
    pub name: String,
    pub age: i32,
    pub department: String,
    pub working: bool,
}

pub async fn connect() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("no db url");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Database failure"))
    // println!("work hard play harder");
}

pub async fn create_employee(Json(payload): Json<CreateEmployee>) -> String {
    let mut conn = connect().await;

    let new_empl = NewAxumEmployee {
        name: &payload.name,
        age: payload.age,
        department: &payload.department,
        working: payload.working,
    };

    diesel::insert_into(axumemployee::table)
        .values(&new_empl)
        .returning(AxumEmployee::as_returning())
        .get_result(&mut conn)
        .expect("insert failed");
    println!("Came here");
    "Employee created".into()
}

pub async fn unwork_employee(Path(id): Path<i32>) -> String {
    use self::schema::axumemployee::dsl::*;

    let mut conn = connect().await;
    let emp_updt = diesel::update(axumemployee.find(id))
        .set(working.eq(true))
        .returning(AxumEmployee::as_returning())
        .get_result(&mut conn)
        .unwrap();

    format!(
        "Update employee ID: {} to working: {}",
        emp_updt.id, emp_updt.working,
    )
}
pub async fn delete_employee(Path(pid): Path<i32>) {
    use self::schema::axumemployee::dsl::*;
    let mut conn = connect().await;
    let emp = diesel::delete(axumemployee.filter(id.eq(pid)))
        .execute(&mut conn)
        .expect("Delete Failed");

    println!("The employee of {id:?} is deleted");
}

pub async fn show_employees() -> String {
    use self::schema::axumemployee::dsl::*;

    let mut conn = connect().await;

    let results = axumemployee
        .filter(working.eq(true))
        .limit(5)
        .select(AxumEmployee::as_select())
        .load(&mut conn)
        .expect("Error loading employee table");

    println!("Displaying {} Employees", results.len());
    serde_json::to_string(&results).unwrap()
}

pub async fn show_employee(Path(sid): Path<i32>) -> String {
    use self::schema::axumemployee::dsl::*;
    println!("Entered id is: {:?}", sid);
    let mut conn = connect().await;
    let emp = axumemployee
        .find(sid)
        .select(AxumEmployee::as_select())
        .first(&mut conn)
        .optional();

    match emp {
        Ok(Some(dude)) => format!("The employee of {:?} is {} ", dude.id, dude.name),
        Ok(None) => format!("Not found"),
        Err(_) => format!("Errored out"),
    }
}
