use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name=crate::schema::axumemployee)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AxumEmployee {
    pub id: i32,
    pub name: String,
    pub age: i32,
    pub department: String,
    pub working: bool,
}

use crate::schema::axumemployee;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name=axumemployee)]
pub struct NewAxumEmployee<'a> {
    pub name: &'a str,
    pub age: i32,
    pub department: &'a str,
    pub working: bool,
}
