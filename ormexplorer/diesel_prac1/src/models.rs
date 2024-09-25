use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name= crate::schema::employee)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub department: String,
    pub working: bool,
}

use crate::schema::employee;

#[derive(Insertable)]
#[diesel(table_name = employee)]
pub struct NewEmployee<'a> {
    pub name: &'a str,
    pub age: i32,
    pub department: &'a str,
    pub working: bool,
}
