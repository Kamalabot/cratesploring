#![allow(unused_imports)]
#![allow(warnings)]

mod models;
mod schema;

use core::panic;
use diesel::{prelude::*, select};
use dotenvy::dotenv;

use models::{Employee, NewEmployee};
use schema::employee;

use std::env;

pub fn connect() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("no db url");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Database failure"))
    // println!("work hard play harder");
}

pub fn create_employee(
    conn: &mut PgConnection,
    name: &str,
    age: i32,
    department: &str,
    working: bool,
) {
    println!("Got name: {name} age: {age} and dept: {department}");
    let new_empl = NewEmployee {
        name,
        age,
        department,
        working,
    };

    diesel::insert_into(employee::table)
        .values(&new_empl)
        .returning(Employee::as_returning())
        .get_result(conn)
        .expect("insert failed");
}

pub fn unwork_employee(conn: &mut PgConnection, id: i32) {
    use self::schema::employee::dsl::*;

    let emp_updt = diesel::update(employee.find(id))
        .set(working.eq(true))
        .returning(Employee::as_returning())
        .get_result(conn)
        .unwrap();

    println!(
        "Update employee ID: {} to working: {}",
        emp_updt.id, emp_updt.working
    )
}

pub fn show_employee(conn: &mut PgConnection, id: i32) {
    use self::schema::employee::dsl::*;

    let emp = employee
        .find(id)
        .select(Employee::as_select())
        .first(conn)
        .optional();

    match emp {
        Ok(Some(dude)) => println!("The employee of {:?} is {} ", dude.id, dude.name),
        Ok(None) => println!("Not found"),
        Err(_) => println!("Errored out"),
    }
}
pub fn delete_employee(conn: &mut PgConnection, id: i32) {
    use self::schema::employee::dsl::*;

    let emp = diesel::delete(employee.find(id))
        .execute(conn)
        .expect("Delete Failed");

    println!("The employee of {id:?} is deleted");
}

pub fn show_employees() {
    use self::schema::employee::dsl::*;

    let conn = &mut connect();

    let results = employee
        .filter(working.eq(true))
        .limit(2)
        .select(Employee::as_select())
        .load(conn)
        .expect("Error loading employee table");

    println!("Displaying {} Employees", results.len());

    for emp in results {
        println!("Name: {}", emp.name);
        println!("-------------------");
        println!("Age: {}", emp.age);
        println!("-------------------");
        println!("department: {}", emp.department);
        println!("-------------------");
        println!("IsWorking: {}", emp.working);
    }
}
