use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name=crate::schema::practrac)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PracTrac {
    pub id: i32,
    pub sessionname: String,
    pub practice: i32,
    pub package: String,
    pub completed: bool,
}

use crate::schema::practrac;

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name=practrac)]
pub struct NewPracTrac {
    pub sessionname: String,
    pub practice: i32,
    pub package: String,
    pub completed: bool,
}
